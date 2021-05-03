using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Xaml;
using TaijuEditor.ViewModels;
using TaijuEditor.Views;
using System;
using System.Diagnostics;

namespace TaijuEditor
{
  public class App : Application
  {
    public override void Initialize()
    {
      AvaloniaXamlLoader.Load(this);
      Trace.Listeners.Add(new TextWriterTraceListener(Console.Out));
    }

    public override void OnFrameworkInitializationCompleted()
    {
      base.OnFrameworkInitializationCompleted();
      if (ApplicationLifetime is IClassicDesktopStyleApplicationLifetime desktop)
      {
        var stage = new TaijuEditor.Models.Stage();
        desktop.MainWindow = new MainWindow
        {
          DataContext = new MainWindowViewModel(stage),
        };
      }
    }
  }
}