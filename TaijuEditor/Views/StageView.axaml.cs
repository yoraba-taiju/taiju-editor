using Avalonia;
using Avalonia.Controls;
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