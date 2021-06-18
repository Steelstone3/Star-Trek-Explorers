using System;
using StarTrek.Contracts.Character;
using StarTrek.Controllers.Game.Character.Factories;

namespace StarTrek.Controllers.Game.Character
{
    public class CrewController : ICrewController
    {
        private ICharacterFactory _characterFactory;

        public CrewController( ICharacterFactory characterFactory)
        {
            _characterFactory = characterFactory;
        }

        public ICrewCompliment CrewCompliment { get; set; } = new CrewCompliment();

        public void AddCrewMember(ICrewRole crewRole, string name)
        {
            var crewMember = _characterFactory.CreateCrewMember(crewRole, name);
            CrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(CrewCompliment, crewMember);
        }

        public void AddCrewMember(ICrewMember crewMember)
        {
            CrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(CrewCompliment, crewMember);
        }
    }
}