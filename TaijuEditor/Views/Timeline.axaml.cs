using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace TaijuEditor.Views
{
  public class Timeline : UserControl
  {
    public Timeline()
    {
      InitializeComponent();
    }

    private void InitializeComponent()
    {
      AvaloniaXamlLoader.Load(this);
    }
  }
}
