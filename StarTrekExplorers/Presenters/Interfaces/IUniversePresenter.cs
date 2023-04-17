using System.Collections.Generic;
using StarTrekExplorersTests.Entities;

namespace StarTrekExplorers.Presenters.Interfaces
{
    public interface IUniversePresenter
    {
        void PrintStars(IEnumerable<IStar> stars);
        void PrintStar(IStar star);
        void PrintPlanet(IPlanet planet);
        void PrintPlanets(IEnumerable<IPlanet> planets);
    }
}