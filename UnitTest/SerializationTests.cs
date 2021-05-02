using Microsoft.VisualStudio.TestTools.UnitTesting;

using TaijuEditor.Models;

namespace UnitTest
{
  [TestClass]
  public class SerializationTests
  {
    [TestMethod]
    public void TestSimple()
    {
      var stage = new TaijuEditor.Models.Stage();
      Assert.AreEqual("{}", stage.Serialize());
    }
  }
}