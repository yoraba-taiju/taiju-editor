using System;
using System.Collections.Generic;
using System.Text;
using Avalonia;
using TaijuEditor.Models;

namespace TaijuEditor.ViewModels
{
  public class MainWindowViewModel : ViewModelBase
  {
    public StageViewModel Stage { get; }

    public MainWindowViewModel(Stage stage)
    {
      Stage = new StageViewModel(stage);
    }
  }
}
