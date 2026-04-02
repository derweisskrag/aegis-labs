---
bibliography: references.bib
geometry:
  - a4paper
  - left=3cm
  - right=2cm
  - top=2.5cm
  - bottom=2.5cm
linestretch: 1.5
header-includes:
  - \usepackage{titlesec}
  - |
    \titleformat{\section}{\fontsize{14pt}{17pt}\selectfont\bfseries\uppercase}{\thesection.}{1em}{}
    \titlespacing*{\section}{0pt}{12pt}{20pt}
    \titleformat{\subsection}{\fontsize{12pt}{15pt}\selectfont\bfseries}{\thesubsection.}{1em}{}
    \titlespacing*{\subsection}{0pt}{12pt}{12pt}
    \titleformat{\subsubsection}{\fontsize{12pt}{15pt}\selectfont\bfseries}{\thesubsubsection.}{1em}{}
    \titlespacing*{\subsubsection}{0pt}{12pt}{12pt}
  - \usepackage{graphicx}
  - \usepackage{times}

  - \usepackage{inconsolata} 
  - \usepackage{soul}
  - \DeclareTextFontCommand{\texttt}{\ttfamily\small}

  - \usepackage[singlelinecheck=false]{caption}
  - |
    \captionsetup[table]{position=above, justification=raggedright}
    \captionsetup[figure]{position=below, justification=raggedright} 
  - \renewcommand{\abstractname}{Kokkuvõte}
  - \renewcommand{\contentsname}{Sisukord}
  - \renewcommand{\figurename}{Joonis}
  - \renewcommand{\tablename}{Tabel}
---

# Kodune projekt nr 1 - programmeerimine  { - }

## 1. Sissejuhatus

Ülesande eesmärk oli süveneda Pythoni programmeerimiskeelde ning luua visuaalselt köitev muster või optiline illusioon. Kuna olen Pythoniga ka varem kokku puutunud, keskendusin eelkõige koodi efektiivsusele ja esteetilisele lõpptulemusele.

## 2. Tööristad

Kasutasin `turtle` teeki, et luua rekursiivne fraktaalpuu. Programm kasutab matemaatilist rekursiooni, kus iga haru tekitab kaks uut alamharu, kuni jõutakse määratud sügavuseni. Visuaalse huvi tekitamiseks rakendasin dünaamilist RGB-värvigammat, mis muutub vastavalt rekursiooni sügavusele, tekitades neoonefektiga valguslahenduse.

```{.render-table data="features"}
```

Selle projekti arhitekrtuur on lihtne, kuid tõhus. Kood on struktureeritud funktsioonideks, mis võimaldab hõlpsat hooldust ja laiendamist. Rekursioon on peamine mehhanism, mis võimaldab luua keerukaid mustreid minimaalse koodiga, samas tagades, et programm jookseb sujuvalt isegi suurema sügavusega:

```{.render-diagram data="architecture_flow"}
```

## 3. Tulemused

Lõpptulemus on visuaalselt köitev fraktaalpuu (`Neon Fractal Tree`), mis meenutab neoonvalgustusega metsa. Värvide dünaamiline muutumine lisab sügavust ja visuaalset huvi, muutes mustri elavaks ja dünaamiliseks. Kood on optimeeritud nii, et see jookseb sujuvalt isegi suurema sügavusega, vältides liigset ressursikasutust.

```{.render-diagram data="python_turtle"}
```

## 4. Koodi analüüs

Tähelepanuväärne on, kuidas rekursioon võimaldab luua keerukaid mustreid minimaalse koodiga. Iga haru tekitab kaks uut alamharu, mis võimaldab luua eksponentsiaalselt kasvava struktuuri, samas kui dünaamiline värvimine lisab visuaalset sügavust ja huvi. Kood on optimeeritud nii, et see jookseb sujuvalt isegi suurema sügavusega, vältides liigset ressursikasutust.

```py
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
```

Main fuktisioon `main()` algatab joonistamise protsessi, seadistades algse haru pikkuse ja käivitades rekursiivse joonistamise:

```py
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
```

Et kasutada koodi, tuleb lihtsalt käivitada `main()` funktsioon, mis algatab kogu joonistamise protsessi: `if __name__ == "__main__": main()`:

```py
if __name__ == "__main__":
    main()
```

Seega on kood struktureeritud nii, et see on lihtne ja arusaadav, samas võimaldades luua keerukaid ja visuaalselt köitvaid mustreid minimaalse koodiga.

## Kokkuvõtte

Selle projekti käigus süvenesin Pythonisse ja õppisin, kuidas luua visuaalselt köitvaid mustreid, kasutades rekursiooni ja dünaamilisi värve. Lõpptulemus on esteetiliselt meeldiv ja näitab, kuidas programmeerimine võib olla kunstiline väljendusvorm. Olen rahul saavutatud tulemusega ning näen suurt potentsiaali edasises programmeerimises ja visuaalsete efektide loomises.


