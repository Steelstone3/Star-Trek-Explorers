using System.Collections.Generic;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Presenters
{
    public class UniversePresenter : IUniversePresenter
    {
        private readonly IPresenter presenter;

        public UniversePresenter(IPresenter presenter)
        {
            this.presenter = presenter;
        }

        public void PrintStars(IEnumerable<IStar> stars)
        {
            foreach (var star in stars)
            {
                PrintStar(star);
            }
        }

        public void PrintStar(IStar star)
        {
            presenter.Print($"\n| Star: {star.Name} {star.StarClass} |");
            PrintPlanets(star.Planets);
        }

        public void PrintPlanets(IEnumerable<IPlanet> planets)
        {
            foreach (var planet in planets)
            {
                PrintPlanet(planet);
            }
        }

        public void PrintPlanet(IPlanet planet)
        {
            presenter.Print($"| Planet: {planet.Name} {planet.PlanetClass} |");
        }
    }
}