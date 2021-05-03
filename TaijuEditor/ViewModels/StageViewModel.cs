using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Text;
using TaijuEditor.Models;

namespace TaijuEditor.ViewModels
{
  public class StageViewModel : ViewModelBase
  {
    private double _progress = 0.0;
    public double Progress
    {
      get => _progress;
      set => this.RaiseAndSetIfChanged(ref _progress, value);
    }
    public Stage Stage { get; }
    public StageViewModel(Stage stage)
    {
      this.Stage = stage;
    }
    
  }
}