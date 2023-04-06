import org.iota.Client;
import org.iota.types.ClientConfig;
import org.iota.types.exceptions.ClientException;
import org.iota.types.MilestonePayload;
import org.iota.types.exceptions.InitializeClientException;
import org.iota.types.ids.MilestoneId;

public class GetMilestoneById {
    public static void main(String[] args) throws ClientException, InitializeClientException {
        // Build the client.
        Client client = new Client(new ClientConfig().withNodes(new String[]{"https://api.testnet.shimmer.network"}));

        // Set up a milestone ID for this example.
        MilestoneId milestoneId = ExampleUtils.setUpMilestoneId(client);

        // Get the milestone.
        MilestonePayload milestone = client.getMilestoneById(milestoneId);

        // Print the milestone.
        System.out.println(milestone);
    }
}