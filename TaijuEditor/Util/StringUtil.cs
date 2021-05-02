using System.Text;

namespace TaijuEditor.Util
{
  public class StringUtil
  {
    public static string ToSnakeCase(string original)
    {
      var sb = new StringBuilder();
      for (var i = 0; i < original.Length; ++i)
      {
        var ch = original[i];
        if (char.IsUpper(ch))
        {
          if (i > 0)
          {
            sb.Append('_');
          }
          sb.Append(char.ToLowerInvariant(ch));
        }
        else
        {
          sb.Append(ch);
        }
      }

      return sb.ToString();
    }
  }
}