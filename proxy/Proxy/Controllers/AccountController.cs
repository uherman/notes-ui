using System.Security.Claims;
using Microsoft.AspNetCore.Authentication;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using StackExchange.Redis;

namespace Proxy.Controllers;

[ApiController]
[Route("[controller]")]
public class AccountController(IConnectionMultiplexer multiplexer) : ControllerBase
{
    private readonly IDatabase _db = multiplexer.GetDatabase();

    [HttpGet("Profile")]
    public IActionResult Profile()
    {
        if (User.Identity?.IsAuthenticated is not true)
        {
            return Unauthorized();
        }

        return new JsonResult(new
        {
            User.Identity.Name,
            User.Identity?.IsAuthenticated
        });
    }

    [HttpPost("Login")]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> Login([FromForm] LoginModel model)
    {
        var username = model.Username;
        var password = model.Password;

        if (string.IsNullOrEmpty(username) || string.IsNullOrEmpty(password))
        {
            return BadRequest("Username and password are required.");
        }

        // Fetch the stored password hash and salt from Redis
        var storedHash = await _db.HashGetAsync($"user:{username}", "passwordHash");
        var storedSalt = await _db.HashGetAsync($"user:{username}", "salt");

        if (storedHash.IsNullOrEmpty || storedSalt.IsNullOrEmpty)
        {
            return Unauthorized("Invalid username or password.");
        }

        var salt = Convert.FromBase64String(storedSalt);
        if (!Auth.VerifyPassword(password, storedHash, salt))
        {
            return Redirect("Account/Login");
        }

        var websSocketToken = Guid.NewGuid();
        await _db.HashSetAsync($"user:{username}", "wsToken", websSocketToken.ToString());
        HttpContext.Response.Cookies.Append("__Host.__ws", websSocketToken.ToString());

        var claims = new List<Claim> { new(ClaimTypes.Name, username) };
        var claimsIdentity = new ClaimsIdentity(claims, Auth.Cookie);
        var authProperties = new AuthenticationProperties { IsPersistent = true };

        await HttpContext.SignInAsync(Auth.Cookie, new ClaimsPrincipal(claimsIdentity), authProperties);
        return Redirect("/");
    }

    [Authorize]
    [HttpPost("Signup")]
    [ValidateAntiForgeryToken]
    public async Task<IActionResult> Signup([FromForm] SignupModel model)
    {
        var username = model.Username;
        var password = model.Password;

        if (string.IsNullOrEmpty(username) || string.IsNullOrEmpty(password))
        {
            return BadRequest("Username and password are required.");
        }

        var existingUser = await _db.HashGetAllAsync($"user:{username}");
        if (existingUser.Length is not 0)
        {
            return Conflict("User already exists.");
        }

        var hashedPassword = Auth.HashPassword(password, out var salt);
        var saltString = Convert.ToBase64String(salt);

        await _db.HashSetAsync($"user:{username}", [
            new HashEntry("passwordHash", hashedPassword),
            new HashEntry("salt", saltString)
        ]);

        return Created("/", "User registered successfully.");
    }

    [HttpPost("Logout")]
    public async Task<IActionResult> Logout()
    {
        await HttpContext.SignOutAsync(Auth.Cookie);
        return Redirect("/Account/Login");
    }

    [HttpGet("AccessDenied")]
    public IActionResult AccessDenied()
    {
        return Content("Access Denied");
    }
}

public record SignupModel(string Username, string Password);

public record LoginModel(string Username, string Password);