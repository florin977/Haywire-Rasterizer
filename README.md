Some images:
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/95cacdaa-5763-4529-9351-d7c35cbc646b" />
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/360a73c1-3e06-45e9-960e-f257453ed36c" />
Depth buffer working (and the coordinate system is fixed, y is not invered anymore)
The y-axis flip caused quite a few problems (most notably: all of the triangles' areas were negative, so I would have needed to flip all the checks. Instead, I changed (1.0 - y) * 0.5 to (y + 1.0) * 0.5 to align with the standard.
<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/8b16618f-a6d7-4c47-913b-06246115aaa4" />
