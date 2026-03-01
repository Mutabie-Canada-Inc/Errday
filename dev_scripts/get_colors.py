from PIL import Image

def get_colors():
    try:
        img = Image.open('docs/images/Matrix.png')
        print(f"Image size: {img.size}")
        
        # Sample background inside the quadrant box (e.g., Do First top left)
        print(f"DO FIRST box bg: {img.getpixel((400, 250))}")
        
        # Sample border color
        print(f"DO FIRST border: {img.getpixel((380, 200))}")
        
        # Sample task card background inside Do First
        print(f"Task bg: {img.getpixel((450, 250))}")
        
        print("Done")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == '__main__':
    get_colors()
