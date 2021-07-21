package com.startrekexplorerstests.game.starship;

import com.startrekexplorers.contracts.IStarship;
import com.startrekexplorers.game.starships.Starship;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

public class StarshipShould {

     private IStarship starship;

    @BeforeEach
    public void setUp() {
        starship = new Starship();
    }

    @Test
    void haveAnInitialPosition() {
        Assertions.assertEquals(1, starship.getXPosition());
        Assertions.assertEquals(1, starship.getYPosition());
    }

    @Test
    void moveUpOnGalaxyMap() {
        starship.moveUp();
        Assertions.assertEquals(1, starship.getXPosition());
        Assertions.assertEquals(2, starship.getYPosition());
    }

    @Test
    void moveDownOnGalaxyMap() {
        starship.moveDown();
        Assertions.assertEquals(1, starship.getXPosition());
        Assertions.assertEquals(0, starship.getYPosition());
    }

    @Test
    void moveLeftOnGalaxyMap() {
        starship.moveLeft();
        Assertions.assertEquals(0, starship.getXPosition());
        Assertions.assertEquals(1, starship.getYPosition());
    }

    @Test
    void moveRight() {
        starship.moveRight();
        Assertions.assertEquals(2, starship.getXPosition());
        Assertions.assertEquals(1, starship.getYPosition());
    }

    @Test
    void notWrapAtTheGalaxyMapLimitsUp() {
        for (int i = 0; i < 20; i++) {
            starship.moveUp();
        }
        Assertions.assertEquals(9, starship.getYPosition());
    }

    @Test
    void notWrapAtTheGalaxyMapLimitsLeft() {
        for (int i = 0; i < 20; i++) {
            starship.moveLeft();
        }
        Assertions.assertEquals(0, starship.getXPosition());
    }

    @Test
    void notWrapAtTheGalaxyMapLimitsDown() {
        for (int i = 0; i < 20; i++) {
            starship.moveDown();
        }
        Assertions.assertEquals(0, starship.getYPosition());
    }

    @Test
    void notWrapAtTheGalaxyMapLimitsRight() {
        for (int i = 0; i < 20; i++) {
            starship.moveRight();
        }
        Assertions.assertEquals(9, starship.getXPosition());
    }

    //Later tests

    @Disabled("For later implementation")
    @Test
    void enterAStarSystem() {

    }

    @Disabled("For later implementation")
    @Test
    void leaveAStarSystem() {

    }

    @Disabled("For later implementation")
    @Test
    void enterAPlanetarySystem() {

    }

    @Disabled("For later implementation")
    @Test
    void leaveAPlanetarySystem() {

    }

    @Disabled("For later implementation")
    @Test
    void enterAMoonSystem() {

    }

    @Disabled("For later implementation")
    @Test
    void leaveAMoonSystem() {
    }

    @Disabled("For later implementation")
    @Test
    void enterAStarBase() {
    }

    @Disabled("For later implementation")
    @Test
    void leaveAStarBase()
    {

    }
}
