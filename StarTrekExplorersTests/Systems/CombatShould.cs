using System.ComponentModel;
using Moq;
using StarTrekExplorers.Components.Interfaces;
using StarTrekExplorers.Entities.Interfaces;
using StarTrekExplorers.Presenters.Interfaces;
using StarTrekExplorers.Systems;
using StarTrekExplorers.Systems.Interfaces;
using Xunit;

namespace StarTrekExplorersTests.Systems
{
    public class CombatShould
    {
        private readonly Mock<IPresenter> presenter = new();
        private readonly Mock<IShipPresenter> shipPresenter = new();
        private readonly Mock<IShip> ship = new();
        private readonly ICombat combat;
        public CombatShould()
        {
            presenter.Setup(p => p.ShipPresenter).Returns(shipPresenter.Object);
            combat = new Combat(presenter.Object);
        }

        [Fact]
        public void StartCombat()
        {
            // Given
            const int damage = 20;
            const int seed = 1234;
            ship.Setup(s => s.DealDamage(seed)).Returns(damage);
            ship.Setup(s => s.TakeDamage(damage));
            shipPresenter.Setup(sp => sp.PrintShipName(ship.Object));
            shipPresenter.Setup(sp => sp.PrintShipOffensiveSystems(ship.Object));
            shipPresenter.Setup(sp => sp.PrintShipDefensiveSystems(ship.Object));

            // When
            combat.Start(seed, ship.Object, ship.Object);

            // Then
            ship.VerifyAll();
            shipPresenter.VerifyAll();
        }
    }
}