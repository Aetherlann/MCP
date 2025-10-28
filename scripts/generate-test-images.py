#!/usr/bin/env python3
"""
Generate test images for Hyper Terminal media testing
"""
import os
import sys

def create_test_images():
    """Create test images using PIL"""
    try:
        from PIL import Image, ImageDraw, ImageFont
    except ImportError:
        print("Error: PIL (Pillow) not installed")
        print("Install with: pip install Pillow")
        return False

    # Create assets directory
    assets_dir = "assets"
    os.makedirs(assets_dir, exist_ok=True)

    # 1. Simple colored square
    print("Creating test-blue.png...")
    img = Image.new('RGB', (200, 200), color='blue')
    draw = ImageDraw.Draw(img)
    draw.text((70, 90), "BLUE", fill='white')
    img.save(f"{assets_dir}/test-blue.png")

    # 2. Gradient image
    print("Creating test-gradient.png...")
    img = Image.new('RGB', (300, 200))
    draw = ImageDraw.Draw(img)
    for x in range(300):
        r = int((x / 300) * 255)
        for y in range(200):
            draw.point((x, y), fill=(r, 100, 255 - r))
    img.save(f"{assets_dir}/test-gradient.png")

    # 3. Pattern image
    print("Creating test-pattern.png...")
    img = Image.new('RGB', (400, 300), color='white')
    draw = ImageDraw.Draw(img)

    # Draw checkerboard
    for i in range(0, 400, 50):
        for j in range(0, 300, 50):
            if (i // 50 + j // 50) % 2 == 0:
                draw.rectangle([i, j, i+50, j+50], fill='black')

    draw.text((150, 140), "Pattern", fill='red')
    img.save(f"{assets_dir}/test-pattern.png")

    # 4. Emoji-like image
    print("Creating test-emoji.png...")
    img = Image.new('RGBA', (150, 150), color=(255, 220, 0, 255))
    draw = ImageDraw.Draw(img)

    # Face
    draw.ellipse([0, 0, 150, 150], fill=(255, 220, 0))

    # Eyes
    draw.ellipse([40, 50, 60, 70], fill='black')
    draw.ellipse([90, 50, 110, 70], fill='black')

    # Smile
    draw.arc([30, 60, 120, 120], 0, 180, fill='black', width=5)

    img.save(f"{assets_dir}/test-emoji.png")

    # 5. Small thumbnail
    print("Creating test-small.png...")
    img = Image.new('RGB', (64, 64), color='green')
    draw = ImageDraw.Draw(img)
    draw.rectangle([10, 10, 54, 54], outline='white', width=3)
    img.save(f"{assets_dir}/test-small.png")

    print("\nTest images created successfully in", assets_dir)
    print("\nYou can now test them with:")
    print("  ./scripts/send-kitty-image.sh assets/test-blue.png")
    return True

if __name__ == "__main__":
    if not create_test_images():
        sys.exit(1)
