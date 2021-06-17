using StarTrek.Controllers.Game.Character;
using StarTrek.Controllers.Game.Character.CrewRoles;
using StarTrek.Controllers.Game.Character.Factories;
using Xunit;

namespace StarTrekTests.Features.Character
{
    public class CharacterFactoryShould
    {
        private CharacterFactory _characterFactory = new CharacterFactory();

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void CreateACaptain(string name)
        {
            var crewRole = new Captain();

            var captain = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(captain);
            Assert.Equal(crewRole.Rank, captain.CrewRole.Rank);
            Assert.Equal(crewRole.Role, captain.CrewRole.Role);
            Assert.Equal(name, captain.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheCaptainToAStarshipCrewCompliment(string name)
        {
            var captainRole = new Captain();
            var captain = new CrewMember(name, captainRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, captain);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.Captain, captain);
        }

        [Theory]
        [InlineData("Bobbington")]
        [InlineData("Charlie")]
        public void CreateAFirstOfficer(string name)
        {
            var crewRole = new FirstOfficer();

            var firstOfficer = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(firstOfficer);
            Assert.Equal(crewRole.Rank, firstOfficer.CrewRole.Rank);
            Assert.Equal(crewRole.Role, firstOfficer.CrewRole.Role);
            Assert.Equal(name, firstOfficer.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheFirstOfficerToAStarshipCrewCompliment(string name)
        {
            var firstOfficerRole = new FirstOfficer();
            var firstOfficer = new CrewMember(name, firstOfficerRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, firstOfficer);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.FirstOfficer, firstOfficer);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Chad")]
        public void CreateAHeadOfEngineering(string name)
        {
            var crewRole = new HeadOfEngineering();

            var headOfEngineering = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(headOfEngineering);
            Assert.Equal(crewRole.Rank, headOfEngineering.CrewRole.Rank);
            Assert.Equal(crewRole.Role, headOfEngineering.CrewRole.Role);
            Assert.Equal(name, headOfEngineering.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheHeadOfEngineeringToAStarshipCrewCompliment(string name)
        {
            var headOfEngineeringRole = new HeadOfEngineering();
            var headOfEngineering = new CrewMember(name, headOfEngineeringRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, headOfEngineering);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.HeadOfEngineering, headOfEngineering);
        }

        [Theory]
        [InlineData("Bobbington")]
        public void CreateAHeadOfSecurity(string name)
        {
            var crewRole = new HeadOfSecurity();

            var headOfSecurity = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(headOfSecurity);
            Assert.Equal(crewRole.Rank, headOfSecurity.CrewRole.Rank);
            Assert.Equal(crewRole.Role, headOfSecurity.CrewRole.Role);
            Assert.Equal(name, headOfSecurity.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheHeadOfSecurityToAStarshipCrewCompliment(string name)
        {
            var headOfSecurityRole = new HeadOfSecurity();
            var headOfSecurity = new CrewMember(name, headOfSecurityRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, headOfSecurity);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.HeadOfSecurity, headOfSecurity);
        }

        [Theory(Skip = "Test not implemented")]
        [InlineData("Bobbington")]
        public void CreateAHeadOfScience(string name)
        {
            var crewRole = new HeadOfScience();

            var headOfScience = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(headOfScience);
            Assert.Equal(crewRole.Rank, headOfScience.CrewRole.Rank);
            Assert.Equal(crewRole.Role, headOfScience.CrewRole.Role);
            Assert.Equal(name, headOfScience.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheHeadOfScienceToAStarshipCrewCompliment(string name)
        {
            var headOfScienceRole = new HeadOfScience();
            var headOfScience = new CrewMember(name, headOfScienceRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, headOfScience);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.HeadOfScience, headOfScience);
        }

        [Theory]
        [InlineData("Bobbington")]
        public void CreateAHeadOfMedical(string name)
        {
            var crewRole = new HeadOfMedical();

            var headOfMedical = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(headOfMedical);
            Assert.Equal(crewRole.Rank, headOfMedical.CrewRole.Rank);
            Assert.Equal(crewRole.Role, headOfMedical.CrewRole.Role);
            Assert.Equal(name, headOfMedical.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheHeadOfMedicalToAStarshipCrewCompliment(string name)
        {
            var headOfMedicalRole = new HeadOfMedical();
            var headOfMedical = new CrewMember(name, headOfMedicalRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, headOfMedical);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.HeadOfMedical, headOfMedical);
        }

        [Theory]
        [InlineData("Bobbington")]
        public void CreateAHeadOfTactical(string name)
        {
            var crewRole = new HeadOfTactical();

            var headOfTactical = _characterFactory.CreateCrewMember(crewRole, name);

            Assert.NotNull(headOfTactical);
            Assert.Equal(crewRole.Rank, headOfTactical.CrewRole.Rank);
            Assert.Equal(crewRole.Role, headOfTactical.CrewRole.Role);
            Assert.Equal(name, headOfTactical.Name);
        }

        [Theory]
        [InlineData("Bobus")]
        [InlineData("Mark The Spark")]
        public void AddTheHeadOfTacticalToAStarshipCrewCompliment(string name)
        {
            var headOfTacticalRole = new HeadOfTactical();
            var headOfTactical = new CrewMember(name, headOfTacticalRole);
            var crewCompliment = new CrewCompliment();

            var newCrewCompliment = _characterFactory.AddCrewMemberToCrewCompliment(crewCompliment, headOfTactical);

            Assert.NotNull(newCrewCompliment);
            Assert.Equal(newCrewCompliment.HeadOfTactical, headOfTactical);
        }
    }
}