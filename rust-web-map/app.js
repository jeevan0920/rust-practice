var map = L.map('map').setView([51.505, -0.09], 13);

L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: 'Map data &copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap contributors</a>',
    maxZoom: 18,
}).addTo(map);

// Function to handle map clicks
function onMapClick(e) {
    var locationName = prompt("Enter a name for this location:", "Unnamed Location");
    if (locationName != null && locationName !== "") {
        var marker = L.marker(e.latlng).addTo(map);
        marker.bindPopup("<b>" + locationName + "</b><br>This is the location you marked.").openPopup();

        // Here we will also send the location data to our backend
        sendLocationToBackend(e.latlng, locationName);
    }
}

function sendLocationToBackend(latlng, locationName) {
    // Placeholder: Replace with your backend's URL
    var backendUrl = 'http://localhost:8080/mark_location';

    // AJAX request to the backend
    var xhr = new XMLHttpRequest();
    xhr.open("POST", backendUrl, true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.onreadystatechange = function () {
        if (xhr.readyState === 4 && xhr.status === 200) {
            console.log("Location saved!");
        }
    };
    var data = JSON.stringify({
        "name": locationName,
        "latitude": latlng.lat,
        "longitude": latlng.lng
    });
    xhr.send(data);
}

map.on('click', onMapClick);