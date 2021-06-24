using Moq;
using StarTrek.Contracts.Character;
using StarTrek.Contracts.Display;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Services.Character;
using Xunit;

namespace StarTrekTests.Features
{
    public class CharacterCreatorServiceShould
    {
        [Fact(Skip = "Test needs implementing")]
        public void CreatePlayerCrew()
        {
            var crewController = new Mock<ICrewController>();
            var genericDisplayHelper = new Mock<IGenericDisplayHelper>();
            var characterCreatorService = new CharacterCreatorService(crewController.Object, genericDisplayHelper.Object);

            var createdCrew = characterCreatorService.CreatePlayerCrew();

            Assert.NotNull(createdCrew);
            genericDisplayHelper.Verify(x => x.GetStringUserInput("Enter Engineer's Name"));
            crewController.Verify(x => x.AddCrewMember(new Captain(), "Bob"));
        }
    }
}