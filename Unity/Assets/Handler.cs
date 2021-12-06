using System.Runtime.InteropServices;
using UnityEngine;

public class Handler : MonoBehaviour
{
    #region DLL Import
    /// <summary>
    /// This imports the <c>generate_random</c> function from our <c>UnityRust</c> DLL.
    /// </summary>
    /// <param name="min_range">Minimum number range</param>
    /// <param name="max_range">Maximum number range</param>
    /// <returns></returns>
    [DllImport("UnityRust")]
    private static extern int generate_random(int min_range, int max_range);
    #endregion

    /// <summary>
    /// Print a random number, grabbed from the Rust DLL.
    /// </summary>
    private void Start() => Debug.Log($"[UnityRust] Your random number is <b>{generate_random(0, 100)}</b>");
}