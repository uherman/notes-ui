using System.Security.Cryptography;
using System.Text;
using Konscious.Security.Cryptography;

namespace Proxy;

public static class Auth
{
    public const string Cookie = "__Host-Notes.Md";

    public static string HashPassword(string password, out byte[] salt)
    {
        salt = new byte[16];
        using (var rng = RandomNumberGenerator.Create())
        {
            rng.GetBytes(salt);
        }

        var argon2 = new Argon2id(Encoding.UTF8.GetBytes(password))
        {
            Salt = salt,
            DegreeOfParallelism = 8,
            Iterations = 4,
            MemorySize = 65536
        };

        var hash = argon2.GetBytes(16);
        return Convert.ToBase64String(hash);
    }

    public static bool VerifyPassword(string password, string storedHash, byte[] salt)
    {
        var argon2 = new Argon2id(Encoding.UTF8.GetBytes(password))
        {
            Salt = salt,
            DegreeOfParallelism = 8,
            Iterations = 4,
            MemorySize = 65536
        };

        var hash = argon2.GetBytes(16);
        var hashString = Convert.ToBase64String(hash);
        return hashString == storedHash;
    }
}