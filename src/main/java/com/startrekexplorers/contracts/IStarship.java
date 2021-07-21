package com.startrekexplorers.contracts;

public interface IStarship {
    int getXPosition();
    int getYPosition();
    void moveUp();
    void moveDown();
    void moveLeft();
    void moveRight();
}