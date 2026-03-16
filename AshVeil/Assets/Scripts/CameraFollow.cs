using UnityEngine;

public class CameraFollow : MonoBehaviour
{
    public Vector3 offset;
    public Transform player;
    private float holdTime = 0f;
    private float repeatTimer = 0f;
    private bool hasRotatedOnce = false;
    void Start()
    {
        offset = transform.position - player.position;
    }

    void CameraInputQ()
    {
        transform.RotateAround(player.position, Vector3.up, -45f);
        offset = transform.position - player.position;
    }

    void CameraInputE()
    {
        transform.RotateAround(player.position, Vector3.up, 45f);
        offset = transform.position - player.position;
    }
    void Update()
    {
    if (Input.GetKey(KeyCode.Q) || Input.GetKey(KeyCode.E))
    {
        holdTime += Time.deltaTime;
        if (holdTime < 0.35f && !hasRotatedOnce)
        {
            if (Input.GetKey(KeyCode.Q))
                CameraInputQ();
            if (Input.GetKey(KeyCode.E))
                CameraInputE();
            hasRotatedOnce = true;
        }

        if (holdTime > 0.35f)
        {
            repeatTimer += Time.deltaTime;
            if (repeatTimer > 0.5f)
            {
                if (Input.GetKey(KeyCode.Q))
                    CameraInputQ();
                if (Input.GetKey(KeyCode.E))
                    CameraInputE();
                repeatTimer = 0f;
            }
        }
    }
    else 
    {
        holdTime = 0f;
        repeatTimer = 0f;
        hasRotatedOnce = false;
    }

    transform.position = player.position + offset;
    }
}
