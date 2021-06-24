using StarTrek.Controllers.Game.Character;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Controllers.Game.Character.Factories;
using Xunit;

namespace StarTrekTests.Features
{
    public class CrewControllerShould
    {

        [Fact]
        public void CreateAndStoreACrewMember()
        {
            var firstOfficer = new FirstOfficer();
            var characterFactory = new CharacterFactory();
            var crewController = new CrewController(characterFactory);

            crewController.AddCrewMember(firstOfficer, "Bob The Slob");

            Assert.NotNull(crewController.CrewCompliment);
            Assert.NotNull(crewController.CrewCompliment.FirstOfficer);
            Assert.Equal("Bob The Slob", crewController.CrewCompliment.FirstOfficer.Name);
            Assert.Equal(crewController.CrewCompliment.FirstOfficer.CrewRole, firstOfficer);

            Assert.Null(crewController.CrewCompliment.Captain);
            Assert.Null(crewController.CrewCompliment.HeadOfEngineering);
            Assert.Null(crewController.CrewCompliment.HeadOfMedical);
            Assert.Null(crewController.CrewCompliment.HeadOfScience);
            Assert.Null(crewController.CrewCompliment.HeadOfSecurity);
            Assert.Null(crewController.CrewCompliment.HeadOfTactical);
        }

        [Fact]
        public void StoreACrewMember()
        {
            var firstOfficer = new FirstOfficer();
            var crewMember = new CrewMember("Bob The Slob", firstOfficer);
            var characterFactory = new CharacterFactory();
            var crewController = new CrewController(characterFactory);

            crewController.AddCrewMember(crewMember);

            Assert.NotNull(crewController.CrewCompliment);
            Assert.NotNull(crewController.CrewCompliment.FirstOfficer);
            Assert.Equal("Bob The Slob", crewController.CrewCompliment.FirstOfficer.Name);
            Assert.Equal(crewController.CrewCompliment.FirstOfficer.CrewRole, firstOfficer);

            Assert.Null(crewController.CrewCompliment.Captain);
            Assert.Null(crewController.CrewCompliment.HeadOfEngineering);
            Assert.Null(crewController.CrewCompliment.HeadOfMedical);
            Assert.Null(crewController.CrewCompliment.HeadOfScience);
            Assert.Null(crewController.CrewCompliment.HeadOfSecurity);
            Assert.Null(crewController.CrewCompliment.HeadOfTactical);
        }

        [Fact(Skip = "Potentially could implement")]
        public void AutoCreateACrewMember()
        {

        }

        [Fact(Skip = "Potentially could implement")]
        public void AutoCreateACrewCompliment()
        {

        }
    }
}