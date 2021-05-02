﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Text.Json;
using System.Threading.Tasks;
using HarfBuzzSharp;
using TaijuEditor.Util;

namespace TaijuEditor.Models
{
  public class Stage
  {
    public World World { get; set; }
    public Stage()
    {
      this.World = new World();
    }

    public string Serialize()
    {
      var serializeOptions = new JsonSerializerOptions
      {
        PropertyNamingPolicy = new ScakeCaseNamingPolicy(),
        WriteIndented = true
      };
      return System.Text.Json.JsonSerializer.Serialize(this, serializeOptions);
    }
  }
}