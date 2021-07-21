package com.StarTrekExplorers.Game.Starships;

import com.StarTrekExplorers.Contracts.IStarship;

public class Starship implements IStarship {
    private int xPosition = 1;
    private int yPosition = 1;

    @Override
    public int getXPosition() {
        return xPosition;
    }

    @Override
    public int getYPosition() {
        return yPosition;
    }

    @Override
    public void moveUp() {
        if (yPosition < 9) {
            yPosition += 1;
        }
    }

    @Override
    public void moveDown() {
        if (yPosition > 0) {
            yPosition -= 1;
        }
    }

    @Override
    public void moveLeft() {
        if (xPosition > 0) {
            xPosition -= 1;
        }
    }

    @Override
    public void moveRight() {
        if (xPosition < 9) {
            xPosition += 1;
        }
    }
}
