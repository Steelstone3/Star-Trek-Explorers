using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Controllers.Game.Character.CrewRoles;

namespace StarTrek.Controllers.Game.Character
{
    public class CrewController : ICrewController
    {
        private IGenericInputHelper _userInput;
        private ICharacterFactory _characterFactory;

        public CrewController(IGenericInputHelper userInput, ICharacterFactory characterFactory)
        {
            _userInput = userInput;
            _characterFactory = characterFactory;
        }

        public ICrewCompliment CrewCompliment { get; set; }

        public void CreateCrew()
        {
            string name = string.Empty;

            _userInput.GetStringUserInput("Enter The Ship Captain's Name");

            var captain = _characterFactory.CreateCrewMember(new Captain(), name);
            CrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(CrewCompliment, captain);

            _userInput.GetStringUserInput("Enter First Officer's Name");

            _characterFactory.CreateCrewMember(new FirstOfficer(), name);

            _userInput.GetStringUserInput("Enter Head Of Engineering's Name");

            _characterFactory.CreateCrewMember(new HeadOfEngineering(), name);
        }
    }
}