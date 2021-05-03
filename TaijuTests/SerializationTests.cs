using NUnit.Framework;
using TaijuEditor;

namespace TaijuTests
{
  public class SerializationTests
  {
    [SetUp]
    public void Setup()
    {
    }

    [Test]
    public void TestSimple()
    {
      var stage = new TaijuEditor.Models.Stage();
      var result = stage.Serialize();
      Assert.AreEqual("{}", result);
      Assert.Pass();
    }
  }
}