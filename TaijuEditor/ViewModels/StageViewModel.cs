using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Text;
using TaijuEditor.Models;

namespace TaijuEditor.ViewModels
{
  public class StageViewModel : ViewModelBase
  {
    public Stage Stage { get; }
    public StageViewModel(Stage stage)
    {
      this.Stage = stage;
    }
    
  }
}