using Microsoft.AspNetCore.DataProtection;
using Microsoft.AspNetCore.HttpOverrides;
using Proxy;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddReverseProxy()
    .LoadFromConfig(builder.Configuration.GetSection("ReverseProxy"));

builder.Services.AddRazorPages();
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();
builder.Services.AddControllers();
builder.Services.AddAntiforgery(options =>
{
    options.Cookie.Name = "__Host-AntiForgery";
    options.Cookie.HttpOnly = true;
    options.Cookie.SecurePolicy = CookieSecurePolicy.Always;
    options.HeaderName = "X-CSRF-TOKEN";
});

builder.Services.AddSingleton(RedisOptions.GetConnectionMultiplexer(builder.Configuration));
var (redis, key) = RedisOptions.GetConnectionMultiplexerDataProtection(builder.Configuration);
builder.Services.AddDataProtection()
    .SetApplicationName("Notes.Md")
    .PersistKeysToStackExchangeRedis(redis, key);

builder.Services.AddAuthentication(Auth.Cookie)
    .AddCookie(Auth.Cookie, options =>
    {
        options.LoginPath = "/Login";
        options.LogoutPath = "/Account/Logout";
        options.AccessDeniedPath = "/Account/AccessDenied";
        options.Cookie.Name = Auth.Cookie;
        options.ExpireTimeSpan = TimeSpan.FromHours(2);
        options.SlidingExpiration = true;
    });

var app = builder.Build();
if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

var forwardedHeadersOptions = new ForwardedHeadersOptions
{
    ForwardedHeaders = ForwardedHeaders.XForwardedFor | ForwardedHeaders.XForwardedProto
};

forwardedHeadersOptions.KnownNetworks.Clear();
forwardedHeadersOptions.KnownProxies.Clear();
    
app.UseForwardedHeaders(forwardedHeadersOptions);
app.UseHsts();
app.UseHttpsRedirection();
app.UseAntiforgery();
app.UseStaticFiles();
app.UseRouting();
app.UseAuthentication();
app.UseAuthorization();
app.MapControllers();
app.MapRazorPages();
app.MapReverseProxy();

app.Run();