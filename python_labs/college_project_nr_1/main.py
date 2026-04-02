import turtle
import random

def draw_branch(branch_len, t):
    if branch_len > 5:
        # Dynamic coloring based on length - gives it a "glow" effect
        t.color(0, max(0, 255 - int(branch_len * 2)), 255)
        # Draw the branch
        t.pensize(branch_len / 10)
        t.forward(branch_len)
        
        # Right sub-branch
        angle = 20 + random.uniform(0, 10)
        t.right(angle)
        draw_branch(branch_len - 15, t)
        
        # Left sub-branch
        t.left(angle * 2)
        draw_branch(branch_len - 15, t)
        
        # Return to node
        t.right(angle)
        t.backward(branch_len)


def main():
    print("[SYSTEM] Initializing Fractal Engine...")
    print(f"[SYSTEM] Recursion depth set. Rendering il buco nero energy... Splendido!")

    turtle.colormode(255) # to use RGB colors
    screen = turtle.Screen()
    screen.bgcolor("black")
    
    t = turtle.Turtle()
    t.speed(0)
    t.left(90)
    t.up()
    t.backward(200)
    t.down()
    t.hideturtle()
    
    # Speed up rendering significantly
    turtle.tracer(0, 0)
    
    draw_branch(100, t)
    
    turtle.update()
    print("Done! Splendido.")
    screen.exitonclick()

if __name__ == "__main__":
    main()