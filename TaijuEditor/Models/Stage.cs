using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using HarfBuzzSharp;

namespace TaijuEditor.Models
{
  public class Stage
  {
    private readonly List<Body> Bodies = new List<Body>();
    public Stage()
    {
    }

    public string Serialize()
    {
      return System.Text.Json.JsonSerializer.Serialize(this);
    }
  }
}
