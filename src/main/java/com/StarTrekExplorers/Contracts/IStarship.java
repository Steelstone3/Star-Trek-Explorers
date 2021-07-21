package com.StarTrekExplorers.Contracts;

public interface IStarship {
    int getXPosition();
    int getYPosition();
    void moveUp();
    void moveDown();
    void moveLeft();
    void moveRight();
}