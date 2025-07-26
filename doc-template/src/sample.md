# Other Projects

---
## Api Tags

<div class="dropdown-container">
<div class="dropdown">
    <div class="dropdown-header" onclick="toggleDropdown(this)">
        <span>Tag Title</span>
        <div class="dropdown-icon"></div>
    </div>
    <div class="dropdown-content">
        <a href="#api-route" class="dropdown-item"><span>Sample</span></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
    </div>
</div>
<div class="dropdown">
    <div class="dropdown-header" onclick="toggleDropdown(this)">
        <span>Tag Title</span>
        <div class="dropdown-icon"></div>
    </div>
    <div class="dropdown-content">
        <a href="#api-route" class="dropdown-item"><span>Sample</span></a>
    </div>
</div>
<div class="dropdown">
    <div class="dropdown-header" onclick="toggleDropdown(this)">
        <span>Tag Title</span>
        <div class="dropdown-icon"></div>
    </div>
    <div class="dropdown-content">
        <a href="#api-route" class="dropdown-item"><span>Sample</span></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
        <a href="#api-route" class="dropdown-item"><div>Sample</div></a>
    </div>
</div>
</div>













---
## Api route
<div class="endpoint">
    <div class="method-post">post</div>
    /admin/user/new   
    <div class="buttons">
        <button class="clip-button" title="Copy to clipboard" aria-label="Copy to clipboard"
        onclick="navigator.clipboard.writeText('https://127.0.0.1:8080/admin/user/new')"
        >
        </button>
    </div>
</div>




### 🔸 Request
<div class="info-body" >
    <h4>Query-String Parameters</h4>
    <p>Descriptions<p>
    <table class="paramter-table">
        <tr class="head">
            <th>Attributes</th>
            <th>Type</th>
            <th>Constraint</th>
            <th>Description</th>
        </tr>
        <tr>
            <td>Nani</td>
            <td>Nani</td>
            <td>
            <div class="constraint-container">
                <div class="constraint">>= 0</div> 
            </div>
            </td>
            <td>NLorem ipsum dolor sit amet, consectetur adipiscing elit. Sed</td>
        </tr>
        <tr>
            <td>Nani</td>
            <td>Nani</td>
            <td>
            <div class="constraint-container">
                <div class="no-constraint">None</div> 
            </div>
            </td>
            <td>NLorem ipsum dolor sit amet, consectetur adipiscing eli</td>
        </tr>
    </table>
</div>

<div class="info-body request-body" id="/api/auth/login" method="post" type="application/json">
    <h4>Path Parameters</h4>
    <p>Descriptions<p>
    <table class="paramter-table">
        <tr class="head">
            <th>Attributes</th>
            <th>Type</th>
            <th>Constraint</th>
            <th>Description</th>
        </tr>
        <tr>
            <td>Nani</td>
            <td>Nani</td>
            <td>
            <div class="constraint-container">
                <div class="constraint">>= 0</div> 
            </div>
            </td>
            <td>NLorem ipsum dolor sit amet, consectetur adipiscing elit. Sed</td>
        </tr>
        <tr>
            <td>Nani</td>
            <td>Nani</td>
            <td>
            <div class="constraint-container">
                <div class="no-constraint">None</div> 
            </div>
            </td>
            <td>NLorem ipsum dolor sit amet, consectetur adipiscing eli</td>
        </tr>
        <tr>
            <td>Nani</td>
            <td>Nani</td>
            <td>
            <div class="constraint-container">
                <div class="no-constraint">None</div> 
            </div>
            </td>
            <td>NLorem ipsu</td>
        </tr>
    </table>
    
</div>

<div class="info-body request-body" id="/api/auth/login" method="post" type="application/json">
    <h4>Request Body</h4>
    <p>Descriptions<p>
    <div class="option-display">
        <div class="option-tab selected" data-tab="schema" onclick="setActiveTab(this)">Schema</div>
        <div class="option-tab" data-tab="example" onclick="setActiveTab(this)">Example</div>
        <div class="underline"></div>
    </div>
    <pre class="content-type">Content Type: application/json</pre>
    <pre><code class="language-json">{
    "email": string,
    "password": string
} 
</code></pre>
</div>


### 🔹 Respond
<div class="response-container" id="/api/auth/login" method="post">
    <div class="response-code">
        <button class="http-200" onclick="response_detail(this,200)">200</button>
        <button class="http-400" onclick="response_detail(this,400)">400</button>
        <button class="http-500" onclick="response_detail(this,500)">501</button>
    </div>
    <div class="info-body">
        <h4>Titlo </h4>
        <pre class="content-type"><p>Content Type: </p><p>application/json<p></pre>
        <pre><code class="language-json">{
    "status": "success" | "fail" | "error"
    "user"?: {
        "id": Uuid,
        "name": string
    },
}
</code></pre>
    </div>
</div>






---
<div class="dropdown-container">
<div class="dropdown">
<div class="dropdown-header" onclick="toggleDropdown(this)">
    <span>Play</span>
    <div class="dropdown-icon"></div>
</div>
<div class="dropdown-content">
    <a href="#user" class="dropdown-item"><span>User</span></a>
<a href="#admin" class="dropdown-item"><span>Admin</span></a>
<a href="#login" class="dropdown-item"><span>Login</span></a>
<a href="#logout" class="dropdown-item"><span>Logout</span></a>
<a href="#update-user" class="dropdown-item"><span>Update User</span></a>
</div>
</div>
<div class="dropdown">
<div class="dropdown-header" onclick="toggleDropdown(this)">
    <span>Service</span>
    <div class="dropdown-icon"></div>
</div>
<div class="dropdown-content">
    <a href="#api-route" class="dropdown-item"><span>Api Route</span></a>
</div>
</div>
<div class="dropdown">
<div class="dropdown-header" onclick="toggleDropdown(this)">
    <span>Backend</span>
    <div class="dropdown-icon"></div>
</div>
<div class="dropdown-content">
    <a href="#auth-verify" class="dropdown-item"><span>Auth Verify</span></a>
<a href="#admin-pull" class="dropdown-item"><span>Admin Pull</span></a>
<a href="#commit" class="dropdown-item"><span>Commit</span></a>
</div>
</div>
</div>
