using UnityEngine;

public class PlayerMovement : MonoBehaviour
{
    public Transform cameraTransform;
    public float speed = 5f;
    void Start()
    {
        if (cameraTransform == null)
        {
            Camera mainCamera = Camera.main;
            if (mainCamera != null)
            {
                cameraTransform = mainCamera.transform;
            }
            else
            {
                Debug.LogError("No main camera found. Please assign cameraTransform manually.");
            }
        }
    }

    void Update()
    {
        float moveX = Input.GetAxis("Horizontal");
        float moveZ = Input.GetAxis("Vertical");
    
        Vector3 forward = cameraTransform.forward;
        forward.y = 0f;
        forward.Normalize();
    
        Vector3 right = cameraTransform.right;
        right.y = 0f;
        right.Normalize();
    
        Vector3 moveDirection = (forward * moveZ + right * moveX);

        // Obróć gracza w kierunku ruchu
        if (moveDirection != Vector3.zero)
        {
            transform.rotation = Quaternion.LookRotation(moveDirection);
        }

        transform.Translate(moveDirection * speed * Time.deltaTime, Space.World);
    }
}
