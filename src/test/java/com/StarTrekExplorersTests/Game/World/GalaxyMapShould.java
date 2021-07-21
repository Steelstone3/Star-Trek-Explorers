package com.StarTrekExplorersTests.Game.World;

import com.StarTrekExplorers.Game.GameMap.GalaxyMap;
import com.StarTrekExplorers.Contracts.IGalaxyMap;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

public class GalaxyMapShould {

    private IGalaxyMap galaxyMap;

    @BeforeEach
    void setUp() {
        galaxyMap = new GalaxyMap();
    }

    @Test
    void generateAGalaxyMapGrid() {
        Assertions.assertNotNull(galaxyMap.getMap());
        Assertions.assertEquals(10, galaxyMap.getMap().length);
        Assertions.assertEquals(10, galaxyMap.getMap()[0].length);
    }

    @Test
    void containStars() {
        Assertions.assertArrayEquals(new char[]{' ', ' ', ' ', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[0]);
        Assertions.assertArrayEquals(new char[]{' ', ' ', ' ', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[1]);
        Assertions.assertArrayEquals(new char[]{' ', ' ', ' ', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[2]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[3]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', '*', '*', '*', ' '}, galaxyMap.getMap()[4]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', ' ', ' ', '*', ' '}, galaxyMap.getMap()[5]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[6]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[7]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[8]);
        Assertions.assertArrayEquals(new char[]{' ', '*', '*', ' ', ' ', '*', ' ', '*', '*', ' '}, galaxyMap.getMap()[9]);
    }

    //Later tests

    @Disabled("For later implementation")
    @Test
    void haveStarsThatContainPlanets() {

    }

    @Disabled("For later implementation")
    @Test
    void haveStarsThatContainStarBases() {

    }

    @Disabled("For later implementation")
    @Test
    void havePlanetsThatContainMoons() {

    }
}
