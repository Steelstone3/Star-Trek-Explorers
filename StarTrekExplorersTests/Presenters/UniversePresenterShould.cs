using System.Collections.Generic;
using Moq;
using StarTrekExplorers.Presenters;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;
using Xunit;

namespace StarTrekExplorersTests.Presenters
{
    public class UniversePresenterShould
    {
        private const string starMessage = "\n| Star: Sol Class A |";
        private const string planetMessage = "| Planet: Earth Class M |";
        private readonly Mock<IStar> star = new();
        private readonly Mock<IPlanet> planet = new();
        private readonly Mock<IPresenter> presenter = new();
        private readonly IUniversePresenter universePresenter;

        public UniversePresenterShould()
        {
            star.Setup(s => s.Name).Returns("Sol");
            star.Setup(s => s.StarClass).Returns("Class A");
            planet.Setup(p => p.Name).Returns("Earth");
            planet.Setup(p => p.PlanetClass).Returns("Class M");
            universePresenter = new UniversePresenter(presenter.Object);
        }

        [Fact]
        public void PrintPlanet()
        {
            // Given
            presenter.Setup(p => p.Print(planetMessage));

            // When
            universePresenter.PrintPlanet(planet.Object);

            // Then
            presenter.VerifyAll();
        }

        [Fact]
        public void PrintPlanets()
        {
            // Given
            List<IPlanet> planets = new() { planet.Object, planet.Object };
            presenter.Setup(p => p.Print(planetMessage));

            // When
            universePresenter.PrintPlanets(planets);

            // Then
            presenter.Verify(p => p.Print(planetMessage), Times.Exactly(2));
        }

        [Fact]
        public void PrintStar()
        {
            // Given
            List<IPlanet> planets = new() { planet.Object, planet.Object };
            star.Setup(s => s.Planets).Returns(planets);
            presenter.Setup(p => p.Print(starMessage));
            presenter.Setup(p => p.Print(planetMessage));

            // When
            universePresenter.PrintStar(star.Object);

            // Then
            presenter.Verify(p => p.Print(starMessage), Times.Exactly(1));
            presenter.Verify(p => p.Print(planetMessage), Times.Exactly(2));
        }

        [Fact]
        public void PrintStars()
        {
            // Given
            List<IPlanet> planets = new() { planet.Object, planet.Object };
            star.Setup(s => s.Planets).Returns(planets);
            List<IStar> stars = new() { star.Object, star.Object };
            presenter.Setup(p => p.Print(starMessage));
            presenter.Setup(p => p.Print(planetMessage));

            // When
            universePresenter.PrintStars(stars);

            // Then
            presenter.Verify(p => p.Print(starMessage), Times.Exactly(2));
            presenter.Verify(p => p.Print(planetMessage), Times.Exactly(4));
        }
    }
}