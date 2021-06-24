using System.Collections.Generic;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Controllers.Game.Character.CrewRoles;

namespace StarTrek.Services.Character
{
    public class CharacterCreatorService : ICharacterCreatorService
    {
        private ICrewController _crewController;
        private IGenericDisplayHelper _genericDisplayHelper;

        public CharacterCreatorService(ICrewController crewController, IGenericDisplayHelper genericDisplayHelper)
        {
            _crewController = crewController;
            _genericDisplayHelper = genericDisplayHelper;
        }

        public ICrewController CreatePlayerCrew()
        {
            string name = string.Empty;

            foreach (var crewRole in CreateCrewRoles())
            {
                string userMessage = $"Enter {crewRole.Role}'s Name";

                name = _genericDisplayHelper.GetStringUserInput(userMessage);
                _crewController.AddCrewMember(crewRole, name);
            }

            return _crewController;
        }

        private IEnumerable<ICrewRole> CreateCrewRoles()
        {
            return new List<ICrewRole>
            {
                new Captain(),
                new FirstOfficer(),
                new HeadOfEngineering(),
                new HeadOfMedical(),
                new HeadOfScience(),
                new HeadOfSecurity(),
                new HeadOfTactical(),
            };
        }
    }
}