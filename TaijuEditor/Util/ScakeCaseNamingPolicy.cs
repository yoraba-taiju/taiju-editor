using System.Text.Json;

namespace TaijuEditor.Util
{
  public class ScakeCaseNamingPolicy : JsonNamingPolicy
  {
    public override string ConvertName(string name)
    {
      return StringUtil.ToSnakeCase(name);
    }
  }
}