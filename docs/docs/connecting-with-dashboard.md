# Connecting Superposition with `euler-api-dashboard`

Follow these steps to run **Superposition** locally and connect it with your `euler-api-dashboard` instance.

---

## 1. Set up and run Superposition locally

First, install all dependencies and start the Superposition service.

You can follow the instructions in the [`setup.md`](./setup.md) of the Superposition repository.

---

## 2. Expose Superposition to your dashboard (if required)

**Do this step only if your `euler-api-dashboard` service is running on a different machine or over SSH** (for example, inside a Podman machine or on a remote host).

Run:

```bash
ssh -R 9000:127.0.0.1:8080 <your-username>@<target-machine-hostname>.local
````

This command:

* **`-R 9000:127.0.0.1:8080`** → Forwards remote port `9000` on the target machine to your local `8080` (where Superposition runs).
* `<your-username>` → The username on the target machine.
* `<target-machine-hostname>.local` → The hostname of the target machine (use `.local` if connecting over mDNS on macOS).

After this, the target machine can reach your local Superposition at:

```
http://localhost:9000
```

Check /health endpoint:

```bash
 # Run this command in your remote machine
 curl --location 'http://localhost:8080/health'
 # Expected Response : "Health is good :D"
```   

---

## 3. Update the dashboard configuration

In your dashboard repo, update the following values:

```haskell
dashboardCACHostUrl :: Text
dashboardCACHostUrl = "http://localhost:9000" -- if required change values accordingly 

dashboardCACOrgId :: Text 
dashboardCACOrgId = "localorg" -- if required change values accordingly 

dashboardCACWorkspaceId :: Text
dashboardCACWorkspaceId = "test" -- if required change values accordingly 
```

---

## 4. Done ✅

Restart your `euler-api-dashboard` service, and it should now communicate with your locally running Superposition instance.
