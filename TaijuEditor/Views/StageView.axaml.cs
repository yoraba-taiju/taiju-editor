using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.Primitives;
using Avalonia.Input;
using Avalonia.Logging;
using Avalonia.Markup.Xaml;

namespace TaijuEditor.Views
{
  public class StageView : UserControl
  {
    public StageView()
    {
      InitializeComponent();
    }

    private void InitializeComponent()
    {
      AvaloniaXamlLoader.Load(this);
    }

  }
}