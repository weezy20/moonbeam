export const moonwallConfig: MoonwallConfig = {
  label: "moonbeamGlobalTest",
  version: "0.23",
  suites: [
    {
      name: "Smoke",
      launchOptions: {
        type: "existing",
        instances: 1,
      },
      providers: [
        {
          name: "ethers",
          type: "ethers",
          endpoint: process.env.WSS_URL,
        },
        {
          name: "relay",
          type: "polkadotJs",
          endpoint: process.env.RELAY_URL,
        },
        {
          name: "parachain",
          type: "polkadotJs",
          endpoint: process.env.WSS_URL,
        },
      ],
      contextAddons: [],
    },

    {
      name: "Dev",
      foundation: {
        type: "Dev",
        instances: ["alith", "balto"],
        // devService: true
      },
      providers: [
        {
          instance: "alith",
          type: "polkadotJs",
          endpoint: process.env.WSS_URL || "new",
        },
      ],
      // contextAddons: ["createBlock", "supplyOptions"],
    },

    // npm run moonwall-test-dev S100 --debug  /** Adds debug options to node binary */
    // ^^ errors if network is incompatible with launching a new node
    // ^^ upto user to be responsible for creating the correct binary (e.g. with symbols added)
    // ^^ interactive mode , inquirer, user prompt before and after test execution

    // npm run moonwall-test-dev S100 --keep-around  /**  */

    {
      name: "AlanTestNetwork1",
      launchOptions: {
        type: "new",
        instances: 1,
        relayConnected: true,
      },
      providers: [
        {
          name: "parachain",
          type: "polkadotJs",
          endpoint: process.env.WSS_URL || "new",
        },
      ],
      contextAddons: ["createBlock", "supplyOptions"],
    },
    {
      name: "ChopsticksTest1",
      launchOptions: {
        type: "chopsticks",
        chopstickConfig: [{ name: "blah", config: "./*.json" }, "./wadawd.yaml", {}],
      },

      providers: [
        {
          name: "parachain",
          type: "polkadotJs",
          endpoint: process.env.WSS_URL || "new",
          contextMethod: ["createBlock"],
        },
        {
          name: "ethers",
          type: "ethers",
          endpoint: process.env.WSS_URL || "new",
          contextMethod: ["sendRawTransaction"],
        },
        {
          name: "acala",
          type: "polkadotJs",
          endpoint: process.env.WSS_ACA_URL || "acala",
        },
        {
          name: "kusama",
          type: "polkadotJs",
          endpoint: process.env.WSS_KSM_URL || "kusama",
        },
      ],
    },
  ],
};

createBlock(provider);

interface MoonwallConfig {
  label: string;
  version: string;
  suites: MoonwallSuite[];
}

interface MoonwallSuite {}
