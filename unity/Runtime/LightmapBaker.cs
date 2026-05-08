#if UNITY_EDITOR
using UnityEngine;

namespace stilb
{
    [ExecuteAlways]
    public class LightmapBaker : MonoBehaviour
    {
        public LightmapGroup globalGroup;
        public LightmapStorage storage;

        void OnEnable()
        {
            if (storage)
            {
                storage.ApplyLightmaps();
            }
        }

    }
}
#endif