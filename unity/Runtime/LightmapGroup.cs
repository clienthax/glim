using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace stilb
{
    [CreateAssetMenu]
    public class LightmapGroup : ScriptableObject
    {
        public uint resolution = 512;
        public uint bounceCount = 5;
        public uint maxSamples = 512;
        public bool denoise = true;
    }
}