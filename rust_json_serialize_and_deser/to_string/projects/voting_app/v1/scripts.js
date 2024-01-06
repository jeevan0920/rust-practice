// Function to handle candidate login
function candidateLogin() {
  const mobileNumber = document.getElementById("mobileNumber").value;

  // Make a POST request to candidate login API
  fetch('http://127.0.0.1:8080/api/candidate/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      mobileNumber: mobileNumber,
    }),
  })
  .then(response => response.json())
  .then(data => {
    if (data.success) {
      // Hide login form and display candidate information
      document.getElementById("candidateLoginForm").style.display = "none";
      document.getElementById("candidateInfo").style.display = "block";

      // Store the token for future API calls
      const token = data.token;

      // Call the function to display candidate information
      displayCandidateInfo(token);
    } else {
      // Handle login failure
      alert(data.message);
    }
  })
  .catch(error => {
    console.error('Error during login:', error);
  });
}

// Function to display candidate information
function displayCandidateInfo(token) {
  // Replace '1' with the actual candidate ID
  const candidateId = '1';

  // Make a GET request to get candidate information
  fetch(`http://127.0.0.1:8080/api/candidate/${candidateId}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  })
  .then(response => response.json())
  .then(candidate => {
    // Display candidate details
    document.getElementById("candidateDetails").innerHTML = `
      <p>ID: ${candidate.id}</p>
      <p>Name: ${candidate.name}</p>
      <img src="${candidate.photo}" alt="Candidate Photo" style="max-width: 200px; max-height: 200px;">
    `;

    // Populate the candidate select dropdown in the vote form
    document.getElementById("candidateId").innerHTML = `
      <option value="${candidate.id}">${candidate.name}</option>
    `;

    // Show vote submission form
    document.getElementById("voteForm").style.display = "block";
  })
  .catch(error => {
    console.error('Error during candidate info retrieval:', error);
  });
}

// Function to submit the vote
function submitVote() {
  const voterId = 'voter_id';  // Replace with actual voter ID
  const candidateId = document.getElementById("candidateId").value;

  // Replace 'voter_access_token' with the actual token obtained during login
  const token = 'voter_access_token';

  // Make a POST request to submit the vote
  fetch('http://127.0.0.1:8080/api/vote', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify({
      voterId: voterId,
      candidateId: candidateId,
      token: token,
    }),
  })
  .then(response => response.json())
  .then(data => {
    if (data.success) {
      // Handle successful vote submission
      alert(data.message);
    } else {
      // Handle vote submission failure
      alert(data.message);
    }
  })
  .catch(error => {
    console.error('Error during vote submission:', error);
  });
}
