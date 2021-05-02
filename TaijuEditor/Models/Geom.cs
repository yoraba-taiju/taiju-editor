using System.Reflection.Metadata;

namespace TaijuEditor.Models
{
  public struct Pos
  {
    public float X;
    public float Y;
    public Pos(float x, float y)
    {
      this.X = x;
      this.Y = y;
    }
  }
  public struct Size
  {
    public float Width;
    public float Height;
    public Size(float w, float h)
    {
      this.Width = w;
      this.Height = h;
    }
}
  public struct Area
  {
    public Pos Center;
    public Size Size;

    public static Area FromLeftBottom(Pos origin, Size size)
    {
      return new Area
      {
        Center = new Pos(origin.X + size.Width / 2.0f, origin.Y + size.Height / 2.0f),
        Size = size,
      };
    }
  }
}