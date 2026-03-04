"""Generate placeholder sprite PNGs for all entity types."""
from PIL import Image, ImageDraw
import os

out = "assets/placeholders"
os.makedirs(out, exist_ok=True)

BASE = 128

SIZES = {
    "small":     (BASE, BASE),
    "medium":    (BASE, BASE * 2),
    "humanoid":  (BASE, BASE * 3),
    "large":     (int(BASE * 1.5), int(BASE * 1.5) * 2),
    "boss":      (BASE * 2, BASE * 2 * 3),
}


def draw_silhouette(draw, w, h, shape, color, outline):
    cx, cy = w // 2, h // 2
    pad = 8

    if shape == "humanoid":
        head_r = w // 6
        head_cy = pad + head_r + 4
        draw.ellipse([cx - head_r, head_cy - head_r, cx + head_r, head_cy + head_r],
                     fill=color, outline=outline, width=2)
        neck_y = head_cy + head_r
        shoulder_w = w // 3
        hip_y = h * 2 // 3
        hip_w = w // 4
        body = [(cx - shoulder_w, neck_y), (cx + shoulder_w, neck_y),
                (cx + hip_w, hip_y), (cx - hip_w, hip_y)]
        draw.polygon(body, fill=color, outline=outline, width=2)
        arm_len = w // 3
        draw.line([(cx - shoulder_w, neck_y + 10),
                   (cx - shoulder_w - arm_len // 2, neck_y + arm_len)],
                  fill=outline, width=4)
        draw.line([(cx + shoulder_w, neck_y + 10),
                   (cx + shoulder_w + arm_len // 2, neck_y + arm_len)],
                  fill=outline, width=4)
        leg_w = 6
        foot_y = h - pad - 4
        draw.rectangle([cx - hip_w + 4, hip_y, cx - hip_w + 4 + leg_w * 2, foot_y],
                       fill=color, outline=outline, width=2)
        draw.rectangle([cx + hip_w - 4 - leg_w * 2, hip_y, cx + hip_w - 4, foot_y],
                       fill=color, outline=outline, width=2)

    elif shape == "beast_small":
        body_w = w // 2
        body_h = h // 3
        body_y = cy - body_h // 2 + h // 8
        draw.ellipse([cx - body_w, body_y, cx + body_w, body_y + body_h],
                     fill=color, outline=outline, width=2)
        head_r = body_h // 3
        draw.ellipse([cx + body_w - head_r, body_y - head_r // 2,
                      cx + body_w + head_r, body_y + head_r + head_r // 2],
                     fill=color, outline=outline, width=2)
        # Eyes
        eye_r = 3
        eye_cx = cx + body_w + head_r // 2
        eye_cy = body_y + head_r // 3
        draw.ellipse([eye_cx - eye_r, eye_cy - eye_r, eye_cx + eye_r, eye_cy + eye_r],
                     fill=(255, 50, 50))
        # Spines on back
        for sx in range(-body_w + 15, body_w - 10, 16):
            spine_x = cx + sx
            draw.polygon([(spine_x - 4, body_y + 2), (spine_x, body_y - 14),
                          (spine_x + 4, body_y + 2)], fill=outline)
        # Legs
        leg_y = body_y + body_h
        for lx in [-body_w + 10, -body_w // 2, body_w // 2, body_w - 10]:
            draw.line([(cx + lx, leg_y), (cx + lx - 4, h - pad)],
                      fill=outline, width=3)

    elif shape == "beast_medium":
        head_r = w // 5
        head_cy = pad + head_r + 10
        draw.ellipse([cx - head_r, head_cy - head_r, cx + head_r, head_cy + head_r],
                     fill=color, outline=outline, width=2)
        # Eyes (glowing)
        for ex in [-head_r // 3, head_r // 3]:
            draw.ellipse([cx + ex - 4, head_cy - 4, cx + ex + 4, head_cy + 4],
                         fill=(255, 200, 0))
        body_top = head_cy + head_r - 4
        body_bot = h * 3 // 4
        body_w = w // 3 + 8
        body = [(cx - body_w, body_top + 20), (cx, body_top),
                (cx + body_w, body_top + 20), (cx + body_w - 8, body_bot),
                (cx - body_w + 8, body_bot)]
        draw.polygon(body, fill=color, outline=outline, width=2)
        # Staff (for shaman)
        staff_x = cx + body_w + 10
        draw.line([(staff_x, body_top), (staff_x, h - pad)], fill=(80, 60, 30), width=4)
        draw.ellipse([staff_x - 8, body_top - 12, staff_x + 8, body_top + 4],
                     fill=(200, 180, 50), outline=(150, 130, 30), width=2)
        # Legs
        draw.rectangle([cx - 16, body_bot, cx - 4, h - pad],
                       fill=color, outline=outline, width=2)
        draw.rectangle([cx + 4, body_bot, cx + 16, h - pad],
                       fill=color, outline=outline, width=2)

    elif shape == "boss":
        head_r = w // 5
        head_cy = pad + head_r + 30
        draw.ellipse([cx - head_r, head_cy - head_r, cx + head_r, head_cy + head_r],
                     fill=color, outline=outline, width=3)
        # Glowing eyes
        for ex in [-head_r // 3, head_r // 3]:
            draw.ellipse([cx + ex - 6, head_cy - 6, cx + ex + 6, head_cy + 6],
                         fill=(255, 50, 50))
        # Horns
        horn_len = head_r + 30
        draw.polygon([(cx - head_r + 4, head_cy - head_r + 4),
                      (cx - head_r - 25, head_cy - horn_len),
                      (cx - head_r + 16, head_cy - head_r + 12)], fill=outline)
        draw.polygon([(cx + head_r - 4, head_cy - head_r + 4),
                      (cx + head_r + 25, head_cy - horn_len),
                      (cx + head_r - 16, head_cy - head_r + 12)], fill=outline)
        # Massive body
        body_top = head_cy + head_r - 8
        body_bot = h * 3 // 4
        shoulder_w = w * 2 // 5
        draw.polygon([(cx - shoulder_w, body_top + 30), (cx, body_top),
                      (cx + shoulder_w, body_top + 30),
                      (cx + shoulder_w - 20, body_bot),
                      (cx - shoulder_w + 20, body_bot)],
                     fill=color, outline=outline, width=3)
        # Spiky shoulders
        for sx_off in [-shoulder_w + 10, shoulder_w - 10]:
            spike_x = cx + sx_off
            draw.polygon([(spike_x - 12, body_top + 30),
                          (spike_x, body_top - 10),
                          (spike_x + 12, body_top + 30)], fill=outline)
        # Arms with claws
        arm_end_y = body_bot - 40
        draw.line([(cx - shoulder_w, body_top + 40),
                   (cx - shoulder_w - 35, arm_end_y)], fill=outline, width=6)
        draw.line([(cx + shoulder_w, body_top + 40),
                   (cx + shoulder_w + 35, arm_end_y)], fill=outline, width=6)
        # Claws
        for claw_x in [cx - shoulder_w - 35, cx + shoulder_w + 35]:
            for angle in [-12, 0, 12]:
                draw.line([(claw_x, arm_end_y),
                           (claw_x + angle, arm_end_y + 20)], fill=outline, width=3)
        # Legs
        leg_w = 24
        draw.rectangle([cx - 35, body_bot, cx - 35 + leg_w, h - pad],
                       fill=color, outline=outline, width=2)
        draw.rectangle([cx + 11, body_bot, cx + 11 + leg_w, h - pad],
                       fill=color, outline=outline, width=2)

    elif shape == "object_barrel":
        barrel_w = w // 3
        barrel_h = h * 2 // 3
        top = cy - barrel_h // 2
        bot = cy + barrel_h // 2
        draw.rectangle([cx - barrel_w, top + 10, cx + barrel_w, bot - 10],
                       fill=color, outline=outline, width=2)
        draw.ellipse([cx - barrel_w, top, cx + barrel_w, top + 20],
                     fill=color, outline=outline, width=2)
        draw.ellipse([cx - barrel_w, bot - 20, cx + barrel_w, bot],
                     fill=color, outline=outline, width=2)
        for by in [top + barrel_h // 3, top + 2 * barrel_h // 3]:
            draw.line([(cx - barrel_w, by), (cx + barrel_w, by)],
                      fill=outline, width=3)

    elif shape == "object_fire":
        log_color = (100, 60, 20)
        draw.line([(cx - 35, h - pad - 12), (cx + 35, h - pad - 12)],
                  fill=log_color, width=8)
        draw.line([(cx - 25, h - pad - 5), (cx + 25, h - pad - 5)],
                  fill=log_color, width=6)
        flame_base_y = h - pad - 18
        flame_layers = [
            ((255, 60, 0), w // 3 + 4, h // 2 + 4),
            ((255, 140, 0), w // 3 - 6, h // 2 - 12),
            ((255, 220, 50), w // 3 - 18, h // 2 - 28),
        ]
        for fc, fw, fh in flame_layers:
            points = [(cx - fw, flame_base_y),
                      (cx - fw // 2, flame_base_y - fh * 2 // 3),
                      (cx, flame_base_y - fh),
                      (cx + fw // 2, flame_base_y - fh * 2 // 3),
                      (cx + fw, flame_base_y)]
            draw.polygon(points, fill=fc)


entities = [
    ("player",        "humanoid", "humanoid",      (60, 100, 180),  (30, 50, 100)),
    ("fallen_shaman", "medium",   "beast_medium",  (120, 50, 30),   (80, 30, 15)),
    ("spike_fiend",   "small",    "beast_small",   (80, 70, 50),    (50, 40, 25)),
    ("andariel",      "boss",     "boss",          (140, 30, 60),   (90, 15, 30)),
    ("akara",         "humanoid", "humanoid",      (150, 120, 80),  (90, 70, 45)),
    ("deckard_cain",  "humanoid", "humanoid",      (100, 80, 140),  (60, 45, 90)),
    ("fire",          "small",    "object_fire",   (255, 160, 30),  (180, 80, 10)),
    ("barrel",        "small",    "object_barrel", (140, 100, 50),  (80, 55, 25)),
]

for name, size_class, shape, color, outline in entities:
    w, h = SIZES[size_class]
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img, "RGBA")
    draw_silhouette(draw, w, h, shape, color, outline)
    path = f"{out}/{name}.png"
    img.save(path)
    print(f"  {name}.png: {w}x{h} ({size_class})")

print(f"\nGenerated {len(entities)} placeholder sprites in {out}/")
