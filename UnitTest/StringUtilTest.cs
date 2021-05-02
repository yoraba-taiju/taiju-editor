using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace UnitTest
{
  [TestClass]
  public class StringUtilTest
  {
    [TestMethod]
    public void TestSnakeCase()
    {
      Assert.AreEqual("test_case", TaijuEditor.Util.StringUtil.ToSnakeCase("TestCase"));
      Assert.AreEqual("test_case", TaijuEditor.Util.StringUtil.ToSnakeCase("testCase"));
    }
  }
}