// Simple HTTP test script
async function testHTTP() {
    console.log('Testing HTTP endpoints...');

    try {
        // Test health endpoint
        const healthResponse = await fetch('http://127.0.0.1:8080/health');
        if (healthResponse.ok) {
            const healthData = await healthResponse.json();
            console.log('✅ Health check:', healthData);
        } else {
            console.error('❌ Health check failed:', healthResponse.status);
        }

        // Test dashboard endpoint
        const dashboardResponse = await fetch('http://127.0.0.1:8080/dashboard');
        if (dashboardResponse.ok) {
            console.log('✅ Dashboard endpoint accessible');
        } else {
            console.error('❌ Dashboard endpoint failed:', dashboardResponse.status);
        }

        // Test status endpoint
        const statusResponse = await fetch('http://127.0.0.1:8080/api/status');
        if (statusResponse.ok) {
            const statusData = await statusResponse.json();
            console.log('✅ Status endpoint:', statusData);
        } else {
            console.error('❌ Status endpoint failed:', statusResponse.status);
        }

    } catch (error) {
        console.error('❌ HTTP request failed:', error);
    }
}

testHTTP();