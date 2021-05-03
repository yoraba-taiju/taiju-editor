using NUnit.Framework;

namespace TaijuTests
{
  public class UtilTests
  {
    [SetUp]
    public void Setup()
    {
    }
    [Test]
    public void TestSnakeCase()
    {
      Assert.AreEqual("test_case", TaijuEditor.Util.StringUtil.ToSnakeCase("TestCase"));
      Assert.AreEqual("test_case", TaijuEditor.Util.StringUtil.ToSnakeCase("testCase"));
    }
  }
}