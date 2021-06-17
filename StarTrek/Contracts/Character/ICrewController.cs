using System.Collections.Generic;
using StarTrek.Controllers.Game.Character;

namespace StarTrek.Contracts.Character
{
    public interface ICrewController
    {
        ICrewCompliment CrewCompliment { get; set; }
        void CreateCrew();
    }
}