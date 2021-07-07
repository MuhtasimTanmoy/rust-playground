# Channel

Data structures needed

- Arc
    - Reference counter to address same object queue in sender, receiver.

- Condvar
    - When it is waiting for it to be awakened it can wake up for unknown reasons by processor.
    - Wait gives up the lock just before it goes to sleep.
    - After wait is finished, the mutex is given back.
     
- Mutex
    - Similar to binary sepaphore with kernel level optimization.
    - Should be given on both sender receiver.


<!DOCTYPE html>
<html lang="en" data-color-mode="auto" data-light-theme="light" data-dark-theme="dark">
  <head>
    <meta charset="utf-8">
  <link rel="dns-prefetch" href="https://github.githubassets.com">
  <link rel="dns-prefetch" href="https://avatars.githubusercontent.com">
  <link rel="dns-prefetch" href="https://github-cloud.s3.amazonaws.com">
  <link rel="dns-prefetch" href="https://user-images.githubusercontent.com/">



  <link crossorigin="anonymous" media="all" integrity="sha512-B/jj6qcXmAwGUh/FG7mfpfFSb0lW1UpGiufFhzIeC+u3lXE5VDEJQzVxZ3gquw8xjZBNQ6CgWDSgCgjRzqPUgw==" rel="stylesheet" href="https://github.githubassets.com/assets/frameworks-07f8e3eaa717980c06521fc51bb99fa5.css" />
  
<link crossorigin="anonymous" media="all" integrity="sha512-pYexK/sPkqbi6Jp00ZyDbs8SMZSduUMxMDMhUUCY616g4axCw/ns7wNWMbcsIPJK54ElTbcHRSMfCPQnHvnuew==" rel="stylesheet" href="https://github.githubassets.com/assets/behaviors-a587b12bfb0f92a6e2e89a74d19c836e.css" />


<link crossorigin="anonymous" media="all" integrity="sha512-hOTL8800D7r7+eJeU+6YWMdw69lO3JLJIt+vRbdgyDu1W2lRPJr1GewyYLCexQhl53OiQgTa4TDr7nZn01TDGg==" rel="stylesheet" href="https://github.githubassets.com/assets/github-84e4cbf3cd340fbafbf9e25e53ee9858.css" />

  <script crossorigin="anonymous" defer="defer" integrity="sha512-CzeY4A6TiG4fGZSWZU8FxmzFFmcQFoPpArF0hkH0/J/S7UL4eed/LKEXMQXfTwiG5yEJBI+9BdKG8KQJNbhcIQ==" type="application/javascript" src="https://github.githubassets.com/assets/environment-0b3798e0.js"></script>
    <script crossorigin="anonymous" defer="defer" integrity="sha512-1nF3VQla2PCUXEjCaUoONz3FfxJIryLvAixrC33H4OlbrvUeQoD1k2sN/+0YtwCi/TNCI89nDAOd5IEWLPWK5A==" type="application/javascript" src="https://github.githubassets.com/assets/chunk-frameworks-d6717755.js"></script>
    <script crossorigin="anonymous" defer="defer" integrity="sha512-3VoJZDGoRhS9kTg9blyG0XFqYY4JXGiD/m4LIsTjIp7L3ZGK0pOVTiQ1YPIP0m7R9cX7+keTq+KBUrS8drfbug==" type="application/javascript" src="https://github.githubassets.com/assets/chunk-vendor-dd5a0964.js"></script>
  
  <script crossorigin="anonymous" defer="defer" integrity="sha512-N2txfSxLmHu6zTzAtk8oXR2NWqEOIgbiaCww9oLETmMphHfYa4fA3HgYtxiMPMM036ATi9EkIqkV8KJvgC3+GQ==" type="application/javascript" src="https://github.githubassets.com/assets/behaviors-376b717d.js"></script>
  
<script crossorigin="anonymous" defer="defer" integrity="sha512-5tWKSr7mhAzSh4Sx5YRFgKftdGxKwHKnOGYw5DlxjHhkQVURYFU3Bk5IMOGMKuAiJTlC3OXYM3xzGcyjzuEFQQ==" type="application/javascript" data-module-id="./chunk-animate-on-scroll.js" data-src="https://github.githubassets.com/assets/chunk-animate-on-scroll-e6d58a4a.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-ct3QiK2mvpg7zor9R2psdWnNMM2K32RU4RGRB/7yA5FyZ8H4iY6SNynXc7UaJqzBx6NaReg3GsWJPwW3kgAAig==" type="application/javascript" data-module-id="./chunk-codemirror.js" data-src="https://github.githubassets.com/assets/chunk-codemirror-72ddd088.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-M6W/sGLOuJXCIkw+doDl6zl7J9q2DmqdwftQCtyEiZM/UJNGRVQdyKwI/PAMxD12se/wCx3ZcyJs9nz0o0OSVw==" type="application/javascript" data-module-id="./chunk-color-modes.js" data-src="https://github.githubassets.com/assets/chunk-color-modes-33a5bfb0.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-71HZu1T5JWqRNF9wrm2NXZAqYVvzxZ8Dvor5U5l/LuEBbGCBX57Sny60Rj+qUZZAvEBGFlNsz179DEn2HFwgVA==" type="application/javascript" data-module-id="./chunk-confetti.js" data-src="https://github.githubassets.com/assets/chunk-confetti-ef51d9bb.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-zkYZSjUFqSifB+Lt76jclFMrfqpcPqevT801RZcoBNCZHRTBKcFrW9OyJoPOzKFv+fZVDRnqdqGsuIv5KOIgZg==" type="application/javascript" data-module-id="./chunk-contributions-spider-graph.js" data-src="https://github.githubassets.com/assets/chunk-contributions-spider-graph-ce46194a.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-6j/oSF+kbW+yetNPvI684VzAu9pzug6Vj2h+3u1LdCuRhR4jnuiHZfeQKls3nxcT/S3H+oIt7FtigE/aeoj+gg==" type="application/javascript" data-module-id="./chunk-drag-drop.js" data-src="https://github.githubassets.com/assets/chunk-drag-drop-ea3fe848.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-VSSd+Yzi2iMS+pibY6hD/WdypxAEdob5F2RMKxuKcAHS2EpFYJPeTXoVxt0NXg03tfj2dka2mEtHS+vjpYSaDw==" type="application/javascript" data-module-id="./chunk-edit-hook-secret-element.js" data-src="https://github.githubassets.com/assets/chunk-edit-hook-secret-element-55249df9.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-N+ziqJjVMfWiqeVHdayDHpNRlG5HsF+cgV+pFnMDoTJuvBzgw+ndsepe4NcKAxIS3WMvzMaQcYmd2vrIaoAJVg==" type="application/javascript" data-module-id="./chunk-edit.js" data-src="https://github.githubassets.com/assets/chunk-edit-37ece2a8.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-aiqMIGGZGo8AQMjcoImKPMTsZVVRl6htCSY7BpRmpGPG/AF+Wq+P/Oj/dthWQOIk9cCNMPEas7O2zAR6oqn0tA==" type="application/javascript" data-module-id="./chunk-emoji-picker-element.js" data-src="https://github.githubassets.com/assets/chunk-emoji-picker-element-6a2a8c20.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-rLdDFAJkSow9YL/I6cWs9Nx790rbDALMvJZ90DQfolTEzxyzh7vEYdM2CrWeCoAaC+aoMQI2btzMFlZ43l5cGA==" type="application/javascript" data-module-id="./chunk-filter-input.js" data-src="https://github.githubassets.com/assets/chunk-filter-input-acb74314.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-Z1wcyOFQHzyMSPqp5DLKrobr3DN2Q6Dz31cfPtw4b2vPs9PX0PrxyDXHpTbIlcZ9qT1M1BNAypHKKw8Lp6Yx/Q==" type="application/javascript" data-module-id="./chunk-insights-graph.js" data-src="https://github.githubassets.com/assets/chunk-insights-graph-675c1cc8.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-ll+uy+JOrHfSzWcUVBwi/jh+C35LRx/QZ0t7P96J1xGcZ/PVhUkUuKkxsJmrCxqt8lL+JwzODQ3RHk310BIv4Q==" type="application/javascript" data-module-id="./chunk-invitations.js" data-src="https://github.githubassets.com/assets/chunk-invitations-965faecb.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-Ffio2herGAoGOqfyPk3iqIXcfjq2dlNCwMqynBMmZnPNhPpc8U3qaojwq+AE27rGPV53V9Usygf+LsWnSZ7qQA==" type="application/javascript" data-module-id="./chunk-jump-to.js" data-src="https://github.githubassets.com/assets/chunk-jump-to-15f8a8da.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-0DSZHh/iD27pAOXl4AWcxPqgRsJAVr1M8SnaN4gKYH2ZplPywvL5oalqN4Qj03FsB5Ll0pytD5kiTMibgGq0BA==" type="application/javascript" data-module-id="./chunk-launch-code-element.js" data-src="https://github.githubassets.com/assets/chunk-launch-code-element-d034991e.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-RduaLAviB2ygvRK/eX5iwzYO43ie7svrJ0rYJs06x7XqpRl/IK8PPBscBWM9Moo5Z86DK2iRLE2+aR7TJ5Uc2Q==" type="application/javascript" data-module-id="./chunk-metric-selection-element.js" data-src="https://github.githubassets.com/assets/chunk-metric-selection-element-45db9a2c.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-Lo0j1owPfYM0txt85KwGzF1PQJLvLFGbRJoASd5ZKMQAV9ZSDg5bVm5UWBAz7glGzw1pkiUD2bCMs2wqyf+CEA==" type="application/javascript" data-module-id="./chunk-notification-list-focus.js" data-src="https://github.githubassets.com/assets/chunk-notification-list-focus-2e8d23d6.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-ma0OOy3nj0c1cqBx0BkcmIFsLqcSZ+MIukQxyEFM/OWTzZpG+QMgOoWPAHZz43M6fyjAUG1jH6c/6LPiiKPCyw==" type="application/javascript" data-module-id="./chunk-profile-pins-element.js" data-src="https://github.githubassets.com/assets/chunk-profile-pins-element-99ad0e3b.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-/kPLzWIe41nxla5A+wcKKPIw4xiAsuITdjbXGVCycmUYnADfCids8FdV+TCA68gr4jAhKIws7flLpL12MoouBA==" type="application/javascript" data-module-id="./chunk-readme-toc-element.js" data-src="https://github.githubassets.com/assets/chunk-readme-toc-element-fe43cbcd.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-nesU0auizp1y0FhtbdzngFVjBVsBEIk/VIVbhC+LcHpGJltFDE7hGwjT8EAbOK5YXTC2cNmheObIukXFtQBtZw==" type="application/javascript" data-module-id="./chunk-ref-selector.js" data-src="https://github.githubassets.com/assets/chunk-ref-selector-9deb14d1.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-Pdw73fp9TN1At4AjDI1042MNWNj/i0OczklFSWkHaUt+d9P1ZlXV/Msu3rHncrs6xRca2WznxDWlgYRsPRyP1w==" type="application/javascript" data-module-id="./chunk-responsive-underlinenav.js" data-src="https://github.githubassets.com/assets/chunk-responsive-underlinenav-3ddc3bdd.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-UUeOf6fdSNCh5PEil2eqo9JMci+9FbJ2YdzZ1rE8fFMYtanaPRRJSidxpPbnl16jxAuQo0QzosPRMKbiFuN0sQ==" type="application/javascript" data-module-id="./chunk-runner-groups.js" data-src="https://github.githubassets.com/assets/chunk-runner-groups-51478e7f.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-tk76eoSLUqXSVZ8ANzPprrOImFIV1zQ/VBV+WzG8ZjZpVPH8cLkMH/ur5HJB1lxx9/yo+V2wjDF96t4qfUwZLA==" type="application/javascript" data-module-id="./chunk-severity-calculator-element.js" data-src="https://github.githubassets.com/assets/chunk-severity-calculator-element-b64efa7a.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-fIq9Mn7jY/bHQXnsmh+VejpDnaO+d/FDxsp+4CuZtdNLrLuO+dQCjh+m6Yd8GCYD2Cy6DWbCEyM+mH2dkB2H9A==" type="application/javascript" data-module-id="./chunk-sortable-behavior.js" data-src="https://github.githubassets.com/assets/chunk-sortable-behavior-7c8abd32.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-WK8VXw3lfUQ/VRW0zlgKPhcMUqH0uTnB/KzePUPdZhCm/HpxfXXHKTGvj5C0Oex7+zbIM2ECzULbtTCT4ug3yg==" type="application/javascript" data-module-id="./chunk-toast.js" data-src="https://github.githubassets.com/assets/chunk-toast-58af155f.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-vgHJEmEJxNmHucGbVY8bEUoOYo5/ZwpQ69rU8Dld89daWJ54uad9lNptxq32F8pnbHhdngw9lohNEbMbjmj5AQ==" type="application/javascript" data-module-id="./chunk-tweetsodium.js" data-src="https://github.githubassets.com/assets/chunk-tweetsodium-be01c912.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-SLqYSMMqthFrVCoXJeZhRyCtWXUsJCUHhj+FJ+bQaBxPTNh+1X0WxCX8u1KQF9eGovmnZUGLEFbUI8PpXhUTXQ==" type="application/javascript" data-module-id="./chunk-user-status-submit.js" data-src="https://github.githubassets.com/assets/chunk-user-status-submit-48ba9848.js"></script>

<script crossorigin="anonymous" defer="defer" integrity="sha512-7lObIUrICZxEDqgAI2LY3ZbDcyCeri5luxBb8niN1q+1DkkUyxKq0Gt9wLRQ1ssDXfELrUnss2dJk50BgUCOXQ==" type="application/javascript" src="https://github.githubassets.com/assets/codespaces-ee539b21.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-HPbX46EFk0YaB4PLTjPkquNCgU+4x94SfypO3VYk+fgp/c76rrA8kt++BRQXugSx+6fXokBKg1GbmgfuW4/cBA==" type="application/javascript" src="https://github.githubassets.com/assets/repositories-1cf6d7e3.js"></script>
<script crossorigin="anonymous" defer="defer" integrity="sha512-VdDnT4MPoFuFNTXGAwYJA7c5Doy8ZNOhM9PaJ0gPsIvXjFpHu9bqjLP6BLa/iRaLY4BXvoo1lQBlQi80vb3WzQ==" type="application/javascript" src="https://github.githubassets.com/assets/topic-suggestions-55d0e74f.js"></script>

  <meta name="viewport" content="width=device-width">
  
  <title>GitHub - bitcoin/bitcoin: Bitcoin Core integration/staging tree</title>
    <meta name="description" content="Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub.">
    <link rel="search" type="application/opensearchdescription+xml" href="/opensearch.xml" title="GitHub">
  <link rel="fluid-icon" href="https://github.com/fluidicon.png" title="GitHub">
  <meta property="fb:app_id" content="1401488693436528">
  <meta name="apple-itunes-app" content="app-id=1477376905" />
    <meta name="twitter:image:src" content="https://opengraph.githubassets.com/d1bafea84bf4320a8c28a296cb97c75a7af04cc4dece4fea51a6c80889bd7ee1/bitcoin/bitcoin" /><meta name="twitter:site" content="@github" /><meta name="twitter:card" content="summary_large_image" /><meta name="twitter:title" content="bitcoin/bitcoin" /><meta name="twitter:description" content="Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub." />
    <meta property="og:image" content="https://opengraph.githubassets.com/d1bafea84bf4320a8c28a296cb97c75a7af04cc4dece4fea51a6c80889bd7ee1/bitcoin/bitcoin" /><meta property="og:image:alt" content="Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub." /><meta property="og:image:width" content="1200" /><meta property="og:image:height" content="600" /><meta property="og:site_name" content="GitHub" /><meta property="og:type" content="object" /><meta property="og:title" content="bitcoin/bitcoin" /><meta property="og:url" content="https://github.com/bitcoin/bitcoin" /><meta property="og:description" content="Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub." />



    

  <link rel="assets" href="https://github.githubassets.com/">
  

  <meta name="request-id" content="E69B:58DA:11A284:16A5D9:60D2FD66" data-pjax-transient="true"/><meta name="html-safe-nonce" content="b64a5ce11d7002fd025c5400860b2402f7d495cb7b2f473b7b713f99c20f65c9" data-pjax-transient="true"/><meta name="visitor-payload" content="eyJyZWZlcnJlciI6IiIsInJlcXVlc3RfaWQiOiJFNjlCOjU4REE6MTFBMjg0OjE2QTVEOTo2MEQyRkQ2NiIsInZpc2l0b3JfaWQiOiIxODc5NTUwMDk5MjA3NjA2ODg5IiwicmVnaW9uX2VkZ2UiOiJhcC1zb3V0aGVhc3QtMSIsInJlZ2lvbl9yZW5kZXIiOiJhcC1zb3V0aGVhc3QtMSJ9" data-pjax-transient="true"/><meta name="visitor-hmac" content="3e22dd86e7df518fa5f2dd9c715aac515b1f9947d788acceb7557553eda9aea2" data-pjax-transient="true"/>

    <meta name="hovercard-subject-tag" content="repository:1181927" data-pjax-transient>


  <meta name="github-keyboard-shortcuts" content="repository" data-pjax-transient="true" />

  

  <meta name="selected-link" value="repo_source" data-pjax-transient>

    <meta name="google-site-verification" content="c1kuD-K2HIVF635lypcsWPoD4kilo5-jA_wBFyT4uMY">
  <meta name="google-site-verification" content="KT5gs8h0wvaagLKAVWq8bbeNwnZZK1r1XQysX3xurLU">
  <meta name="google-site-verification" content="ZzhVyEFwb7w3e0-uOTltm8Jsck2F5StVihD0exw2fsA">
  <meta name="google-site-verification" content="GXs5KoUUkNCoaAZn7wPN-t01Pywp9M3sEjnt_3_ZWPc">

  <meta name="octolytics-host" content="collector.githubapp.com" /><meta name="octolytics-app-id" content="github" /><meta name="octolytics-event-url" content="https://collector.githubapp.com/github-external/browser_event" />

  <meta name="analytics-location" content="/&lt;user-name&gt;/&lt;repo-name&gt;" data-pjax-transient="true" />

  



  <meta name="optimizely-datafile" content="{&quot;version&quot;: &quot;4&quot;, &quot;rollouts&quot;: [], &quot;typedAudiences&quot;: [], &quot;anonymizeIP&quot;: true, &quot;projectId&quot;: &quot;16737760170&quot;, &quot;variables&quot;: [], &quot;featureFlags&quot;: [], &quot;experiments&quot;: [{&quot;status&quot;: &quot;Running&quot;, &quot;audienceIds&quot;: [], &quot;variations&quot;: [{&quot;variables&quot;: [], &quot;id&quot;: &quot;20227754799&quot;, &quot;key&quot;: &quot;control&quot;}, {&quot;variables&quot;: [], &quot;id&quot;: &quot;20233267869&quot;, &quot;key&quot;: &quot;treatment&quot;}], &quot;id&quot;: &quot;20194668672&quot;, &quot;key&quot;: &quot;recommended_plan_in_signup&quot;, &quot;layerId&quot;: &quot;20231804245&quot;, &quot;trafficAllocation&quot;: [{&quot;entityId&quot;: &quot;20233267869&quot;, &quot;endOfRange&quot;: 2500}, {&quot;entityId&quot;: &quot;&quot;, &quot;endOfRange&quot;: 5000}, {&quot;entityId&quot;: &quot;20227754799&quot;, &quot;endOfRange&quot;: 7500}, {&quot;entityId&quot;: &quot;&quot;, &quot;endOfRange&quot;: 10000}], &quot;forcedVariations&quot;: {&quot;d0c8cbf56b61c99517936207d280de0c&quot;: &quot;treatment&quot;}}, {&quot;status&quot;: &quot;Running&quot;, &quot;audienceIds&quot;: [], &quot;variations&quot;: [{&quot;variables&quot;: [], &quot;id&quot;: &quot;20233300304&quot;, &quot;key&quot;: &quot;launch_code_variation&quot;}, {&quot;variables&quot;: [], &quot;id&quot;: &quot;20227370325&quot;, &quot;key&quot;: &quot;control&quot;}], &quot;id&quot;: &quot;20206000276&quot;, &quot;key&quot;: &quot;launch_code_verification&quot;, &quot;layerId&quot;: &quot;20233240262&quot;, &quot;trafficAllocation&quot;: [{&quot;entityId&quot;: &quot;20233300304&quot;, &quot;endOfRange&quot;: 2500}, {&quot;entityId&quot;: &quot;20227370325&quot;, &quot;endOfRange&quot;: 5000}, {&quot;entityId&quot;: &quot;&quot;, &quot;endOfRange&quot;: 10000}], &quot;forcedVariations&quot;: {}}], &quot;audiences&quot;: [{&quot;conditions&quot;: &quot;[\&quot;or\&quot;, {\&quot;match\&quot;: \&quot;exact\&quot;, \&quot;name\&quot;: \&quot;$opt_dummy_attribute\&quot;, \&quot;type\&quot;: \&quot;custom_attribute\&quot;, \&quot;value\&quot;: \&quot;$opt_dummy_value\&quot;}]&quot;, &quot;id&quot;: &quot;$opt_dummy_audience&quot;, &quot;name&quot;: &quot;Optimizely-Generated Audience for Backwards Compatibility&quot;}], &quot;groups&quot;: [], &quot;attributes&quot;: [{&quot;id&quot;: &quot;16822470375&quot;, &quot;key&quot;: &quot;user_id&quot;}, {&quot;id&quot;: &quot;17143601254&quot;, &quot;key&quot;: &quot;spammy&quot;}, {&quot;id&quot;: &quot;18175660309&quot;, &quot;key&quot;: &quot;organization_plan&quot;}, {&quot;id&quot;: &quot;18813001570&quot;, &quot;key&quot;: &quot;is_logged_in&quot;}, {&quot;id&quot;: &quot;19073851829&quot;, &quot;key&quot;: &quot;geo&quot;}, {&quot;id&quot;: &quot;20175462351&quot;, &quot;key&quot;: &quot;requestedCurrency&quot;}], &quot;botFiltering&quot;: false, &quot;accountId&quot;: &quot;16737760170&quot;, &quot;events&quot;: [{&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;17911811441&quot;, &quot;key&quot;: &quot;hydro_click.dashboard.teacher_toolbox_cta&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18124116703&quot;, &quot;key&quot;: &quot;submit.organizations.complete_sign_up&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18145892387&quot;, &quot;key&quot;: &quot;no_metric.tracked_outside_of_optimizely&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18178755568&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.add_repo&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18180553241&quot;, &quot;key&quot;: &quot;submit.repository_imports.create&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18186103728&quot;, &quot;key&quot;: &quot;click.help.learn_more_about_repository_creation&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18188530140&quot;, &quot;key&quot;: &quot;test_event.do_not_use_in_production&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18191963644&quot;, &quot;key&quot;: &quot;click.empty_org_repo_cta.transfer_repository&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18195612788&quot;, &quot;key&quot;: &quot;click.empty_org_repo_cta.import_repository&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18210945499&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.invite_members&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18211063248&quot;, &quot;key&quot;: &quot;click.empty_org_repo_cta.create_repository&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18215721889&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.update_profile&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18224360785&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.dismiss&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18234832286&quot;, &quot;key&quot;: &quot;submit.organization_activation.complete&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18252392383&quot;, &quot;key&quot;: &quot;submit.org_repository.create&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18257551537&quot;, &quot;key&quot;: &quot;submit.org_member_invitation.create&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18259522260&quot;, &quot;key&quot;: &quot;submit.organization_profile.update&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18564603625&quot;, &quot;key&quot;: &quot;view.classroom_select_organization&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18568612016&quot;, &quot;key&quot;: &quot;click.classroom_sign_in_click&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18572592540&quot;, &quot;key&quot;: &quot;view.classroom_name&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18574203855&quot;, &quot;key&quot;: &quot;click.classroom_create_organization&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18582053415&quot;, &quot;key&quot;: &quot;click.classroom_select_organization&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18589463420&quot;, &quot;key&quot;: &quot;click.classroom_create_classroom&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18591323364&quot;, &quot;key&quot;: &quot;click.classroom_create_first_classroom&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18591652321&quot;, &quot;key&quot;: &quot;click.classroom_grant_access&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18607131425&quot;, &quot;key&quot;: &quot;view.classroom_creation&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;18831680583&quot;, &quot;key&quot;: &quot;upgrade_account_plan&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19064064515&quot;, &quot;key&quot;: &quot;click.signup&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19075373687&quot;, &quot;key&quot;: &quot;click.view_account_billing_page&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19077355841&quot;, &quot;key&quot;: &quot;click.dismiss_signup_prompt&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19079713938&quot;, &quot;key&quot;: &quot;click.contact_sales&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19120963070&quot;, &quot;key&quot;: &quot;click.compare_account_plans&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19151690317&quot;, &quot;key&quot;: &quot;click.upgrade_account_cta&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19424193129&quot;, &quot;key&quot;: &quot;click.open_account_switcher&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19520330825&quot;, &quot;key&quot;: &quot;click.visit_account_profile&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19540970635&quot;, &quot;key&quot;: &quot;click.switch_account_context&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19730198868&quot;, &quot;key&quot;: &quot;submit.homepage_signup&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19820830627&quot;, &quot;key&quot;: &quot;click.homepage_signup&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19988571001&quot;, &quot;key&quot;: &quot;click.create_enterprise_trial&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20036538294&quot;, &quot;key&quot;: &quot;click.create_organization_team&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20040653299&quot;, &quot;key&quot;: &quot;click.input_enterprise_trial_form&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20062030003&quot;, &quot;key&quot;: &quot;click.continue_with_team&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20068947153&quot;, &quot;key&quot;: &quot;click.create_organization_free&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20086636658&quot;, &quot;key&quot;: &quot;click.signup_continue.username&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20091648988&quot;, &quot;key&quot;: &quot;click.signup_continue.create_account&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20103637615&quot;, &quot;key&quot;: &quot;click.signup_continue.email&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20111574253&quot;, &quot;key&quot;: &quot;click.signup_continue.password&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20120044111&quot;, &quot;key&quot;: &quot;view.pricing_page&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20152062109&quot;, &quot;key&quot;: &quot;submit.create_account&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20165800992&quot;, &quot;key&quot;: &quot;submit.upgrade_payment_form&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20171520319&quot;, &quot;key&quot;: &quot;submit.create_organization&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20222645674&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.discuss_your_needs&quot;}, {&quot;experimentIds&quot;: [&quot;20206000276&quot;], &quot;id&quot;: &quot;20227443657&quot;, &quot;key&quot;: &quot;submit.verify_primary_user_email&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20234607160&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.try_enterprise&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20238175784&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.team&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20239847212&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.continue_free&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20251097193&quot;, &quot;key&quot;: &quot;recommended_plan&quot;}], &quot;revision&quot;: &quot;699&quot;}" />
  <!-- To prevent page flashing, the optimizely JS needs to be loaded in the
    <head> tag before the DOM renders -->
  <script crossorigin="anonymous" defer="defer" integrity="sha512-yDmmCGyENqEePvF9X9A4omxWCNcbS6qK2h8HZPdnvXd02Vzhtqd0zRd/pgTgqQ/xOA02F3H225VpJvDXrnfNbA==" type="application/javascript" src="https://github.githubassets.com/assets/optimizely-c839a608.js"></script>



  

      <meta name="hostname" content="github.com">
    <meta name="user-login" content="">


      <meta name="expected-hostname" content="github.com">


    <meta name="enabled-features" content="MARKETPLACE_PENDING_INSTALLATIONS">

  <meta http-equiv="x-pjax-version" content="87f8658c32da34e071cd8c5cc1982a2958ea21e57a2527dc58c4ff59fdb6c0ab">
  

    
  <meta name="go-import" content="github.com/bitcoin/bitcoin git https://github.com/bitcoin/bitcoin.git">

  <meta name="octolytics-dimension-user_id" content="528860" /><meta name="octolytics-dimension-user_login" content="bitcoin" /><meta name="octolytics-dimension-repository_id" content="1181927" /><meta name="octolytics-dimension-repository_nwo" content="bitcoin/bitcoin" /><meta name="octolytics-dimension-repository_public" content="true" /><meta name="octolytics-dimension-repository_is_fork" content="false" /><meta name="octolytics-dimension-repository_network_root_id" content="1181927" /><meta name="octolytics-dimension-repository_network_root_nwo" content="bitcoin/bitcoin" />



    <link rel="canonical" href="https://github.com/bitcoin/bitcoin" data-pjax-transient>


  <meta name="browser-stats-url" content="https://api.github.com/_private/browser/stats">

  <meta name="browser-errors-url" content="https://api.github.com/_private/browser/errors">

  <meta name="browser-optimizely-client-errors-url" content="https://api.github.com/_private/browser/optimizely_client/errors">

  <link rel="mask-icon" href="https://github.githubassets.com/pinned-octocat.svg" color="#000000">
  <link rel="alternate icon" class="js-site-favicon" type="image/png" href="https://github.githubassets.com/favicons/favicon.png">
  <link rel="icon" class="js-site-favicon" type="image/svg+xml" href="https://github.githubassets.com/favicons/favicon.svg">

<meta name="theme-color" content="#1e2327">
<meta name="color-scheme" content="light dark" />


  <link rel="manifest" href="/manifest.json" crossOrigin="use-credentials">

<meta name="enabled-homepage-translation-languages" content="">

  </head>

  <body class="logged-out env-production page-responsive" style="word-wrap: break-word;">
    

    <div class="position-relative js-header-wrapper ">
      <a href="#start-of-content" class="px-2 py-4 color-bg-info-inverse color-text-white show-on-focus js-skip-to-content">Skip to content</a>
      <span data-view-component="true" class="progress-pjax-loader width-full js-pjax-loader-bar Progress position-fixed">
    <span style="background-color: #79b8ff;width: 0%;" data-view-component="true" class="Progress-item progress-pjax-loader-bar"></span>
</span>      
      


        
            <header class="Header-old header-logged-out js-details-container Details position-relative f4 py-2" role="banner">
  <div class="container-xl d-lg-flex flex-items-center p-responsive">
    <div class="d-flex flex-justify-between flex-items-center">
        <a class="mr-4" href="https://github.com/" aria-label="Homepage" data-ga-click="(Logged out) Header, go to homepage, icon:logo-wordmark">
          <svg height="32" class="octicon octicon-mark-github color-text-white" viewBox="0 0 16 16" version="1.1" width="32" aria-hidden="true"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
        </a>

          <div class="d-lg-none css-truncate css-truncate-target width-fit p-2">
            

          </div>

        <div class="d-flex flex-items-center">
              <a href="/signup?ref_cta=Sign+up&amp;ref_loc=header+logged+out&amp;ref_page=%2F%3Cuser-name%3E%2F%3Crepo-name%3E&amp;source=header-repo"
                class="d-inline-block d-lg-none f5 color-text-white no-underline border color-border-tertiary rounded-2 px-2 py-1 mr-3 mr-sm-5"
                data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;site header&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;SIGN_UP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="eede25cdbef5dc840192e10fcde2597001ef5002d277864f39a27424031daa40"
              >
                Sign&nbsp;up
              </a>

          <button class="btn-link d-lg-none mt-1 js-details-target" type="button" aria-label="Toggle navigation" aria-expanded="false">
            <svg height="24" class="octicon octicon-three-bars color-text-white" viewBox="0 0 16 16" version="1.1" width="24" aria-hidden="true"><path fill-rule="evenodd" d="M1 2.75A.75.75 0 011.75 2h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 2.75zm0 5A.75.75 0 011.75 7h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 7.75zM1.75 12a.75.75 0 100 1.5h12.5a.75.75 0 100-1.5H1.75z"></path></svg>
          </button>
        </div>
    </div>

    <div class="HeaderMenu HeaderMenu--logged-out position-fixed top-0 right-0 bottom-0 height-fit position-lg-relative d-lg-flex flex-justify-between flex-items-center flex-auto">
      <div class="d-flex d-lg-none flex-justify-end border-bottom color-bg-secondary p-3">
        <button class="btn-link js-details-target" type="button" aria-label="Toggle navigation" aria-expanded="false">
          <svg height="24" class="octicon octicon-x color-text-secondary" viewBox="0 0 24 24" version="1.1" width="24" aria-hidden="true"><path fill-rule="evenodd" d="M5.72 5.72a.75.75 0 011.06 0L12 10.94l5.22-5.22a.75.75 0 111.06 1.06L13.06 12l5.22 5.22a.75.75 0 11-1.06 1.06L12 13.06l-5.22 5.22a.75.75 0 01-1.06-1.06L10.94 12 5.72 6.78a.75.75 0 010-1.06z"></path></svg>
        </button>
      </div>

        <nav class="mt-0 px-3 px-lg-0 mb-5 mb-lg-0" aria-label="Global">
          <ul class="d-lg-flex list-style-none">
              <li class="d-block d-lg-flex flex-lg-nowrap flex-lg-items-center border-bottom border-lg-bottom-0 mr-0 mr-lg-3 edge-item-fix position-relative flex-wrap flex-justify-between d-flex flex-items-center ">
                <details class="HeaderMenu-details details-overlay details-reset width-full">
                  <summary class="HeaderMenu-summary HeaderMenu-link px-0 py-3 border-0 no-wrap d-block d-lg-inline-block">
                    Why GitHub?
                    <svg x="0px" y="0px" viewBox="0 0 14 8" xml:space="preserve" fill="none" class="icon-chevon-down-mktg position-absolute position-lg-relative">
                      <path d="M1,1l6.2,6L13,1"></path>
                    </svg>
                  </summary>
                  <div class="dropdown-menu flex-auto rounded px-0 mt-0 pb-4 p-lg-4 position-relative position-lg-absolute left-0 left-lg-n4">
                    <a href="/features" class="py-2 lh-condensed-ultra d-block Link--primary no-underline h5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Features">Features <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a>
                    <ul class="list-style-none f5 pb-3">


                          <li class="edge-item-fix"><a href="/mobile" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Mobile <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/actions" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Actions <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/codespaces" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Codespaces <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/packages" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Packages <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/security" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Security <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/code-review/" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Code review <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/project-management/" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Project management <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                          <li class="edge-item-fix"><a href="/features/integrations" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">Integrations <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>


                    </ul>

                    <ul class="list-style-none mb-0 border-lg-top pt-lg-3">
                      <li class="edge-item-fix"><a href="/sponsors" class="py-2 lh-condensed-ultra d-block no-underline Link--primary no-underline h5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Sponsors">GitHub Sponsors <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="/customer-stories" class="py-2 lh-condensed-ultra d-block no-underline Link--primary no-underline h5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Customer stories">Customer stories<span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                    </ul>
                  </div>
                </details>
              </li>
              <li class="border-bottom border-lg-bottom-0 mr-0 mr-lg-3">
                <a href="/team" class="HeaderMenu-link no-underline py-3 d-block d-lg-inline-block" data-ga-click="(Logged out) Header, go to Team">Team</a>
              </li>
              <li class="border-bottom border-lg-bottom-0 mr-0 mr-lg-3">
                <a href="/enterprise" class="HeaderMenu-link no-underline py-3 d-block d-lg-inline-block" data-ga-click="(Logged out) Header, go to Enterprise">Enterprise</a>
              </li>

              <li class="d-block d-lg-flex flex-lg-nowrap flex-lg-items-center border-bottom border-lg-bottom-0 mr-0 mr-lg-3 edge-item-fix position-relative flex-wrap flex-justify-between d-flex flex-items-center ">
                <details class="HeaderMenu-details details-overlay details-reset width-full">
                  <summary class="HeaderMenu-summary HeaderMenu-link px-0 py-3 border-0 no-wrap d-block d-lg-inline-block">
                    Explore
                    <svg x="0px" y="0px" viewBox="0 0 14 8" xml:space="preserve" fill="none" class="icon-chevon-down-mktg position-absolute position-lg-relative">
                      <path d="M1,1l6.2,6L13,1"></path>
                    </svg>
                  </summary>

                  <div class="dropdown-menu flex-auto rounded px-0 pt-2 pb-0 mt-0 pb-4 p-lg-4 position-relative position-lg-absolute left-0 left-lg-n4">
                    <ul class="list-style-none mb-3">
                      <li class="edge-item-fix"><a href="/explore" class="py-2 lh-condensed-ultra d-block Link--primary no-underline h5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Explore">Explore GitHub <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                    </ul>

                    <h4 class="color-text-tertiary text-normal text-mono f5 mb-2 border-lg-top pt-lg-3">Learn and contribute</h4>
                    <ul class="list-style-none mb-3">
                      <li class="edge-item-fix"><a href="/topics" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Topics">Topics <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                        <li class="edge-item-fix"><a href="/collections" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Collections">Collections <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="/trending" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Trending">Trending <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://lab.github.com/" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Learning lab">Learning Lab <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://opensource.guide" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Open source guides">Open source guides <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                    </ul>

                    <h4 class="color-text-tertiary text-normal text-mono f5 mb-2 border-lg-top pt-lg-3">Connect with others</h4>
                    <ul class="list-style-none mb-0">
                      <li class="edge-item-fix"><a href="https://github.com/readme" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover">The ReadME Project <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://github.com/events" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Events">Events <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://github.community" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Community forum">Community forum <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://education.github.com" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to GitHub Education">GitHub Education <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://stars.github.com" class="py-2 pb-0 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to GitHub Stars Program">GitHub Stars program <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                    </ul>
                  </div>
                </details>
              </li>

              <li class="border-bottom border-lg-bottom-0 mr-0 mr-lg-3">
                <a href="/marketplace" class="HeaderMenu-link no-underline py-3 d-block d-lg-inline-block" data-ga-click="(Logged out) Header, go to Marketplace">Marketplace</a>
              </li>

              <li class="d-block d-lg-flex flex-lg-nowrap flex-lg-items-center border-bottom border-lg-bottom-0 mr-0 mr-lg-3 edge-item-fix position-relative flex-wrap flex-justify-between d-flex flex-items-center ">
                <details class="HeaderMenu-details details-overlay details-reset width-full">
                  <summary class="HeaderMenu-summary HeaderMenu-link px-0 py-3 border-0 no-wrap d-block d-lg-inline-block">
                    Pricing
                    <svg x="0px" y="0px" viewBox="0 0 14 8" xml:space="preserve" fill="none" class="icon-chevon-down-mktg position-absolute position-lg-relative">
                       <path d="M1,1l6.2,6L13,1"></path>
                    </svg>
                  </summary>

                  <div class="dropdown-menu flex-auto rounded px-0 pt-2 pb-4 mt-0 p-lg-4 position-relative position-lg-absolute left-0 left-lg-n4">
                    <a href="/pricing" class="pb-2 lh-condensed-ultra d-block Link--primary no-underline h5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Pricing">Plans <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a>

                    <ul class="list-style-none mb-3">
                      <li class="edge-item-fix"><a href="/pricing#feature-comparison" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Compare plans">Compare plans <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                      <li class="edge-item-fix"><a href="https://enterprise.github.com/contact" class="py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover" data-ga-click="(Logged out) Header, go to Contact Sales">Contact Sales <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                    </ul>

                    <ul class="list-style-none mb-0 border-lg-top pt-lg-3">
                      <li class="edge-item-fix"><a href="https://education.github.com" class="py-2 pb-0 lh-condensed-ultra d-block no-underline Link--primary no-underline h5 Bump-link--hover"  data-ga-click="(Logged out) Header, go to Education">Education <span class="Bump-link-symbol float-right text-normal color-text-tertiary pr-3">&rarr;</span></a></li>
                    </ul>
                  </div>
                </details>
              </li>
          </ul>
        </nav>

      <div class="d-lg-flex flex-items-center px-3 px-lg-0 text-center text-lg-left">
          <div class="d-lg-flex min-width-0 mb-3 mb-lg-0">
            


  <div class="header-search flex-auto js-site-search position-relative flex-self-stretch flex-md-self-auto mb-3 mb-md-0 mr-0 mr-md-3 scoped-search site-scoped-search js-jump-to"
  >
    <div class="position-relative">
      <!-- '"` --><!-- </textarea></xmp> --></option></form><form class="js-site-search-form" role="search" aria-label="Site" data-scope-type="Repository" data-scope-id="1181927" data-scoped-search-url="/bitcoin/bitcoin/search" data-owner-scoped-search-url="/orgs/bitcoin/search" data-unscoped-search-url="/search" action="/bitcoin/bitcoin/search" accept-charset="UTF-8" method="get">
        <label class="form-control input-sm header-search-wrapper p-0 js-chromeless-input-container header-search-wrapper-jump-to position-relative d-flex flex-justify-between flex-items-center">
          <input type="text"
            class="form-control input-sm header-search-input jump-to-field js-jump-to-field js-site-search-focus js-site-search-field is-clearable"
            data-hotkey=s,/
            name="q"
            value=""
            placeholder="Search"
            data-unscoped-placeholder="Search GitHub"
            data-scoped-placeholder="Search"
            autocapitalize="off"
            role="combobox"
            aria-haspopup="listbox"
            aria-expanded="false"
            aria-autocomplete="list"
            aria-controls="jump-to-results"
            aria-label="Search"
            data-jump-to-suggestions-path="/_graphql/GetSuggestedNavigationDestinations"
            spellcheck="false"
            autocomplete="off"
          >
          <input type="hidden" data-csrf="true" class="js-data-jump-to-suggestions-path-csrf" value="CU1SHGdIVr/7jCQhGlT4oxGnv4J5ZOzy8n/Cr9mSAu5aphL5SkvpL1oTr8blBqEoURWhHi/j8Ff36hyz9HGGfA==" />
          <input type="hidden" class="js-site-search-type-field" name="type" >
              <img src="https://github.githubassets.com/images/search-key-slash.svg" alt="" class="mr-2 header-search-key-slash">

            <div class="Box position-absolute overflow-hidden d-none jump-to-suggestions js-jump-to-suggestions-container">
              
<ul class="d-none js-jump-to-suggestions-template-container">
  

<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-suggestion" role="option">
  <a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="" data-item-type="suggestion">
    <div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
      <svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
    </div>

    <img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">

    <div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
    </div>

    <div class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search">
      <span class="js-jump-to-badge-search-text-default d-none" aria-label="in this repository">
        In this repository
      </span>
      <span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
        All GitHub
      </span>
      <span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>

    <div aria-hidden="true" class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
      Jump to
      <span class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>
  </a>
</li>

</ul>

<ul class="d-none js-jump-to-no-results-template-container">
  <li class="d-flex flex-justify-center flex-items-center f5 d-none js-jump-to-suggestion p-2">
    <span class="color-text-secondary">No suggested jump to results</span>
  </li>
</ul>

<ul id="jump-to-results" role="listbox" class="p-0 m-0 js-navigation-container jump-to-suggestions-results-container js-jump-to-suggestions-results-container">
  

<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-scoped-search d-none" role="option">
  <a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="" data-item-type="scoped_search">
    <div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
      <svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
    </div>

    <img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">

    <div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
    </div>

    <div class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search">
      <span class="js-jump-to-badge-search-text-default d-none" aria-label="in this repository">
        In this repository
      </span>
      <span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
        All GitHub
      </span>
      <span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>

    <div aria-hidden="true" class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
      Jump to
      <span class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>
  </a>
</li>

  

<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-owner-scoped-search d-none" role="option">
  <a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="" data-item-type="owner_scoped_search">
    <div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
      <svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
    </div>

    <img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">

    <div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
    </div>

    <div class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search">
      <span class="js-jump-to-badge-search-text-default d-none" aria-label="in this organization">
        In this organization
      </span>
      <span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
        All GitHub
      </span>
      <span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>

    <div aria-hidden="true" class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
      Jump to
      <span class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>
  </a>
</li>

  

<li class="d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-global-search d-none" role="option">
  <a tabindex="-1" class="no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2" href="" data-item-type="global_search">
    <div class="jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none">
      <svg height="16" width="16" class="octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none" title="Repository" aria-label="Repository" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none" title="Project" aria-label="Project" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
      <svg height="16" width="16" class="octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none" title="Search" aria-label="Search" viewBox="0 0 16 16" version="1.1" role="img"><path fill-rule="evenodd" d="M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z"></path></svg>
    </div>

    <img class="avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none" alt="" aria-label="Team" src="" width="28" height="28">

    <div class="jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target">
    </div>

    <div class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search">
      <span class="js-jump-to-badge-search-text-default d-none" aria-label="in this repository">
        In this repository
      </span>
      <span class="js-jump-to-badge-search-text-global d-none" aria-label="in all of GitHub">
        All GitHub
      </span>
      <span aria-hidden="true" class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>

    <div aria-hidden="true" class="border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump">
      Jump to
      <span class="d-inline-block ml-1 v-align-middle">↵</span>
    </div>
  </a>
</li>


</ul>

            </div>
        </label>
</form>    </div>
  </div>

          </div>

        <div class="position-relative mr-3">
          <a href="/login?return_to=%2Fbitcoin%2Fbitcoin"
            class="HeaderMenu-link flex-shrink-0 no-underline"
            data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;site header menu&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;SIGN_UP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="644092c20f517603530d2054f8f0fa1898daddbfb04c9da7fcd7b8ad8ba103ca"
            data-ga-click="(Logged out) Header, clicked Sign in, text:sign-in">
            Sign in
          </a>
        </div>

            <a href="/signup?ref_cta=Sign+up&amp;ref_loc=header+logged+out&amp;ref_page=%2F%3Cuser-name%3E%2F%3Crepo-name%3E&amp;source=header-repo&amp;source_repo=bitcoin%2Fbitcoin"
              class="HeaderMenu-link flex-shrink-0 d-inline-block no-underline border color-border-tertiary rounded px-2 py-1"
              data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;site header menu&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;SIGN_UP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="644092c20f517603530d2054f8f0fa1898daddbfb04c9da7fcd7b8ad8ba103ca"
              data-hydro-click="{&quot;event_type&quot;:&quot;analytics.event&quot;,&quot;payload&quot;:{&quot;category&quot;:&quot;Sign up&quot;,&quot;action&quot;:&quot;click to sign up for account&quot;,&quot;label&quot;:&quot;ref_page:/&lt;user-name&gt;/&lt;repo-name&gt;;ref_cta:Sign up;ref_loc:header logged out&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="c1787dae27c29f3ef7c3a1fdae1614d04f34ed70f05c5e7597273c3e7048efb1"
            >
              Sign up
            </a>
      </div>
    </div>
  </div>
</header>

    </div>

  <div id="start-of-content" class="show-on-focus"></div>





    <div data-pjax-replace id="js-flash-container">


  <template class="js-flash-template">
    <div class="flash flash-full  {{ className }}">
  <div class=" px-2" >
    <button class="flash-close js-flash-close" type="button" aria-label="Dismiss this message">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-x">
    <path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path>
</svg>
    </button>
    
      <div>{{ message }}</div>

  </div>
</div>
  </template>
</div>


    

  <include-fragment class="js-notification-shelf-include-fragment" data-base-src="https://github.com/notifications/beta/shelf"></include-fragment>




  <div
    class="application-main "
    data-commit-hovercards-enabled
    data-discussion-hovercards-enabled
    data-issue-and-pr-hovercards-enabled
  >
        <div itemscope itemtype="http://schema.org/SoftwareSourceCode" class="">
    <main id="js-repo-pjax-container" data-pjax-container >
      

      
    






  


  <div class="hx_page-header-bg pt-3 hide-full-screen mb-5">

      <div class="d-flex mb-3 px-3 px-md-4 px-lg-5">

        <div class="flex-auto min-width-0 width-fit mr-3">
            <h1 class=" d-flex flex-wrap flex-items-center break-word f3 text-normal">
    <svg class="octicon octicon-repo color-text-secondary mr-2" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z"></path></svg>
  <span class="author flex-self-stretch" itemprop="author">
    <a class="url fn" rel="author" data-hovercard-type="organization" data-hovercard-url="/orgs/bitcoin/hovercard" href="/bitcoin">bitcoin</a>
  </span>
  <span class="mx-1 flex-self-stretch color-text-secondary">/</span>
  <strong itemprop="name" class="mr-2 flex-self-stretch">
    <a data-pjax="#js-repo-pjax-container" href="/bitcoin/bitcoin">bitcoin</a>
  </strong>
  
</h1>


        </div>

          <ul class="pagehead-actions flex-shrink-0 d-none d-md-inline" style="padding: 2px 0;">

  <li>
      <a class="tooltipped tooltipped-s btn btn-sm" aria-label="You must be signed in to change notification settings" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;notification subscription menu watch&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="e10ea9b99c78014bc22224bdb805ecdc9d06f57b1acce6a5430a56f1cfa6aa7d" href="/login?return_to=%2Fbitcoin%2Fbitcoin">
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-bell">
    <path d="M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z"></path><path fill-rule="evenodd" d="M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z"></path>
</svg>
    Notifications
</a>
  </li>

  <li>
          <a class="btn btn-sm btn-with-count  tooltipped tooltipped-s" aria-label="You must be signed in to star a repository" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;star button&quot;,&quot;repository_id&quot;:1181927,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="063182d321a46eaef39541cfa8541885c6ec9053971b8659c00a4e5a865e8603" href="/login?return_to=%2Fbitcoin%2Fbitcoin">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-star v-align-text-bottom mr-1">
    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path>
</svg>
      <span data-view-component="true">
        Star
</span></a>
    <a class="social-count js-social-count" href="/bitcoin/bitcoin/stargazers"
      aria-label="55211 users starred this repository">
      55.2k
    </a>

  </li>

  <li>
        <a class="btn btn-sm btn-with-count tooltipped tooltipped-s" aria-label="You must be signed in to fork a repository" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;repo details fork button&quot;,&quot;repository_id&quot;:1181927,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="8b549d22536b537c0103309c960956b6f5289ce501f9b885723bd746a0e785f1" href="/login?return_to=%2Fbitcoin%2Fbitcoin">
          <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-repo-forked">
    <path fill-rule="evenodd" d="M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z"></path>
</svg>
          Fork
</a>
      <a href="/bitcoin/bitcoin/network/members" class="social-count"
         aria-label="29215 users forked this repository">
        29.2k
      </a>
  </li>
</ul>

      </div>
          <div class="d-block d-md-none mb-2 px-3 px-md-4 px-lg-5">
      <p class="f4 mb-3">
        Bitcoin Core integration/staging tree
      </p>
      <div class="mb-2 d-flex flex-items-center">
        <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-link flex-shrink-0 mr-2">
    <path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path>
</svg>
        <span class="flex-auto min-width-0 css-truncate css-truncate-target width-fit">
          <a title="https://bitcoincore.org/en/download" role="link" target="_blank" class="text-bold" rel="noopener noreferrer" href="https://bitcoincore.org/en/download">bitcoincore.org/en/download</a>
        </span>
      </div>
      <div class="mb-2">
        <a href="/bitcoin/bitcoin/blob/master/COPYING" class="Link--muted">
          <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-law mr-1">
    <path fill-rule="evenodd" d="M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z"></path>
</svg>
            MIT License
        </a>
      </div>
    <div class="mb-3">
      <a class="Link--secondary no-underline mr-3" href="/bitcoin/bitcoin/stargazers">
        <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-star mr-1">
    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path>
</svg>
        <span class="text-bold">55.2k</span>
        stars
</a>      <a class="Link--secondary no-underline" href="/bitcoin/bitcoin/network/members">
        <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-repo-forked mr-1">
    <path fill-rule="evenodd" d="M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z"></path>
</svg>
        <span class="text-bold">29.2k</span>
        forks
</a>    </div>
    <div class="d-flex">
      <div class="flex-1 mr-2">
            <a class="btn btn-sm  btn-block tooltipped tooltipped-s" aria-label="You must be signed in to star a repository" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;star button&quot;,&quot;repository_id&quot;:1181927,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="063182d321a46eaef39541cfa8541885c6ec9053971b8659c00a4e5a865e8603" href="/login?return_to=%2Fbitcoin%2Fbitcoin">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-star v-align-text-bottom mr-1">
    <path fill-rule="evenodd" d="M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z"></path>
</svg>
      <span data-view-component="true">
        Star
</span></a>

      </div>
      <div class="flex-1">
          <a class="tooltipped tooltipped-s btn btn-sm btn-block" aria-label="You must be signed in to change notification settings" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;notification subscription menu watch&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="e10ea9b99c78014bc22224bdb805ecdc9d06f57b1acce6a5430a56f1cfa6aa7d" href="/login?return_to=%2Fbitcoin%2Fbitcoin">
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-bell">
    <path d="M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z"></path><path fill-rule="evenodd" d="M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z"></path>
</svg>
    Notifications
</a>
      </div>
    </div>
  </div>

        

  <nav data-pjax="#js-repo-pjax-container" aria-label="Repository" data-view-component="true" class="js-repo-nav js-sidenav-container-pjax js-responsive-underlinenav overflow-hidden UnderlineNav px-3 px-md-4 px-lg-5">

    <ul data-view-component="true" class="UnderlineNav-body list-style-none">
        <li data-view-component="true" class="d-flex">
  <a href="/bitcoin/bitcoin" data-tab-item="i0code-tab" data-selected-links="repo_source repo_downloads repo_commits repo_releases repo_tags repo_branches repo_packages repo_deployments /bitcoin/bitcoin" data-hotkey="g c" data-ga-click="Repository, Navigation click, Code tab" aria-current="page" data-view-component="true" class="UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item selected">
    
                  <svg class="octicon octicon-code UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z"></path></svg>
          <span data-content="Code">Code</span>
            <span title="Not available" data-view-component="true" class="Counter"></span>

    
</a></li>
        <li data-view-component="true" class="d-flex">
  <a href="/bitcoin/bitcoin/issues" data-tab-item="i1issues-tab" data-selected-links="repo_issues repo_labels repo_milestones /bitcoin/bitcoin/issues" data-hotkey="g i" data-ga-click="Repository, Navigation click, Issues tab" data-view-component="true" class="UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item">
    
                  <svg class="octicon octicon-issue-opened UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3z"></path><path fill-rule="evenodd" d="M8 0a8 8 0 100 16A8 8 0 008 0zM1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0z"></path></svg>
          <span data-content="Issues">Issues</span>
            <span title="588" data-view-component="true" class="Counter">588</span>

    
</a></li>
        <li data-view-component="true" class="d-flex">
  <a href="/bitcoin/bitcoin/pulls" data-tab-item="i2pull-requests-tab" data-selected-links="repo_pulls checks /bitcoin/bitcoin/pulls" data-hotkey="g p" data-ga-click="Repository, Navigation click, Pull requests tab" data-view-component="true" class="UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item">
    
                  <svg class="octicon octicon-git-pull-request UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z"></path></svg>
          <span data-content="Pull requests">Pull requests</span>
            <span title="402" data-view-component="true" class="Counter">402</span>

    
</a></li>
        <li data-view-component="true" class="d-flex">
  <a href="/bitcoin/bitcoin/projects" data-tab-item="i3projects-tab" data-selected-links="repo_projects new_repo_project repo_project /bitcoin/bitcoin/projects" data-hotkey="g b" data-ga-click="Repository, Navigation click, Projects tab" data-view-component="true" class="UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item">
    
                  <svg class="octicon octicon-project UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z"></path></svg>
          <span data-content="Projects">Projects</span>
            <span title="7" data-view-component="true" class="Counter">7</span>

    
</a></li>
        <li data-view-component="true" class="d-flex">
  <a href="/bitcoin/bitcoin/security" data-tab-item="i4security-tab" data-selected-links="security overview alerts policy token_scanning code_scanning /bitcoin/bitcoin/security" data-hotkey="g s" data-ga-click="Repository, Navigation click, Security tab" data-view-component="true" class="UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item">
    
                  <svg class="octicon octicon-shield UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.467.133a1.75 1.75 0 011.066 0l5.25 1.68A1.75 1.75 0 0115 3.48V7c0 1.566-.32 3.182-1.303 4.682-.983 1.498-2.585 2.813-5.032 3.855a1.7 1.7 0 01-1.33 0c-2.447-1.042-4.049-2.357-5.032-3.855C1.32 10.182 1 8.566 1 7V3.48a1.75 1.75 0 011.217-1.667l5.25-1.68zm.61 1.429a.25.25 0 00-.153 0l-5.25 1.68a.25.25 0 00-.174.238V7c0 1.358.275 2.666 1.057 3.86.784 1.194 2.121 2.34 4.366 3.297a.2.2 0 00.154 0c2.245-.956 3.582-2.104 4.366-3.298C13.225 9.666 13.5 8.36 13.5 7V3.48a.25.25 0 00-.174-.237l-5.25-1.68zM9 10.5a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.75a.75.75 0 10-1.5 0v3a.75.75 0 001.5 0v-3z"></path></svg>
          <span data-content="Security">Security</span>
            <include-fragment src="/bitcoin/bitcoin/security/overall-count" accept="text/fragment+html"></include-fragment>

    
</a></li>
        <li data-view-component="true" class="d-flex">
  <a href="/bitcoin/bitcoin/pulse" data-tab-item="i5insights-tab" data-selected-links="repo_graphs repo_contributors dependency_graph dependabot_updates pulse people community /bitcoin/bitcoin/pulse" data-ga-click="Repository, Navigation click, Insights tab" data-view-component="true" class="UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item">
    
                  <svg class="octicon octicon-graph UnderlineNav-octicon d-none d-sm-inline" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M1.5 1.75a.75.75 0 00-1.5 0v12.5c0 .414.336.75.75.75h14.5a.75.75 0 000-1.5H1.5V1.75zm14.28 2.53a.75.75 0 00-1.06-1.06L10 7.94 7.53 5.47a.75.75 0 00-1.06 0L3.22 8.72a.75.75 0 001.06 1.06L7 7.06l2.47 2.47a.75.75 0 001.06 0l5.25-5.25z"></path></svg>
          <span data-content="Insights">Insights</span>
            <span title="Not available" data-view-component="true" class="Counter"></span>

    
</a></li>
</ul>
      <div style="visibility:hidden;" data-view-component="true" class="UnderlineNav-actions  js-responsive-underlinenav-overflow position-absolute pr-3 pr-md-4 pr-lg-5 right-0">      <details data-view-component="true" class="details-overlay details-reset position-relative">
  <summary role="button" data-view-component="true">          <div class="UnderlineNav-item mr-0 border-0">
            <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-kebab-horizontal">
    <path d="M8 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM1.5 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm13 0a1.5 1.5 0 100-3 1.5 1.5 0 000 3z"></path>
</svg>
            <span class="sr-only">More</span>
          </div>
</summary>
  <div data-view-component="true">          <details-menu role="menu" data-view-component="true" class="dropdown-menu dropdown-menu-sw">
  
            <ul>
                <li data-menu-item="i0code-tab" hidden>
                  <a role="menuitem" class="js-selected-navigation-item selected dropdown-item" aria-current="page" data-selected-links=" /bitcoin/bitcoin" href="/bitcoin/bitcoin">
                    Code
</a>                </li>
                <li data-menu-item="i1issues-tab" hidden>
                  <a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bitcoin/bitcoin/issues" href="/bitcoin/bitcoin/issues">
                    Issues
</a>                </li>
                <li data-menu-item="i2pull-requests-tab" hidden>
                  <a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bitcoin/bitcoin/pulls" href="/bitcoin/bitcoin/pulls">
                    Pull requests
</a>                </li>
                <li data-menu-item="i3projects-tab" hidden>
                  <a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bitcoin/bitcoin/projects" href="/bitcoin/bitcoin/projects">
                    Projects
</a>                </li>
                <li data-menu-item="i4security-tab" hidden>
                  <a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bitcoin/bitcoin/security" href="/bitcoin/bitcoin/security">
                    Security
</a>                </li>
                <li data-menu-item="i5insights-tab" hidden>
                  <a role="menuitem" class="js-selected-navigation-item dropdown-item" data-selected-links=" /bitcoin/bitcoin/pulse" href="/bitcoin/bitcoin/pulse">
                    Insights
</a>                </li>
            </ul>

</details-menu></div>
</details></div>
</nav>

  </div>


<div class="container-xl clearfix new-discussion-timeline px-3 px-md-4 px-lg-5">
  <div id="repo-content-pjax-container" class="repository-content " >

    
      
    <div>
  <div class="d-none d-lg-block mt-6 mr-3 Popover top-0 right-0 color-shadow-medium col-3">
    
  </div>


  <div data-view-component="true" class="gutter-condensed gutter-lg flex-column flex-md-row d-flex">

  <div data-view-component="true" class="flex-shrink-0 col-12 col-md-9 mb-4 mb-md-0">      





      <div class="file-navigation mb-3 d-flex flex-items-start">
  
<div class="position-relative">
  <details class="details-reset details-overlay mr-0 mb-0 " id="branch-select-menu">
    <summary class="btn css-truncate"
            data-hotkey="w"
            title="Switch branches or tags">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-git-branch text-gray">
    <path fill-rule="evenodd" d="M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z"></path>
</svg>
      <span class="css-truncate-target" data-menu-button>master</span>
      <span class="dropdown-caret"></span>
    </summary>

      
<div class="SelectMenu">
  <div class="SelectMenu-modal">
    <header class="SelectMenu-header">
      <span class="SelectMenu-title">Switch branches/tags</span>
      <button class="SelectMenu-closeButton" type="button" data-toggle-for="branch-select-menu"><svg aria-label="Close menu" aria-hidden="false" role="img" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-x">
    <path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path>
</svg></button>
    </header>

    <input-demux data-action="tab-container-change:input-demux#storeInput tab-container-changed:input-demux#updateInput">
      <tab-container class="d-flex flex-column js-branches-tags-tabs" style="min-height: 0;">
        <div class="SelectMenu-filter">
          <input data-target="input-demux.source"
                 id="context-commitish-filter-field"
                 class="SelectMenu-input form-control"
                 aria-owns="ref-list-branches"
                 data-controls-ref-menu-id="ref-list-branches"
                 autofocus
                 autocomplete="off"
                 aria-label="Filter branches/tags"
                 placeholder="Filter branches/tags"
                 type="text"
          >
        </div>

        <div class="SelectMenu-tabs" role="tablist" data-target="input-demux.control" >
          <button class="SelectMenu-tab" type="button" role="tab" aria-selected="true">Branches</button>
          <button class="SelectMenu-tab" type="button" role="tab">Tags</button>
        </div>

        <div role="tabpanel" id="ref-list-branches" data-filter-placeholder="Filter branches/tags" class="d-flex flex-column flex-auto overflow-auto" tabindex="">
          <ref-selector
            type="branch"
            data-targets="input-demux.sinks"
            data-action="
              input-entered:ref-selector#inputEntered
              tab-selected:ref-selector#tabSelected
              focus-list:ref-selector#focusFirstListMember
            "
            query-endpoint="/bitcoin/bitcoin/refs"
            
            cache-key="v0:1622739464.0582771"
            current-committish="bWFzdGVy"
            default-branch="bWFzdGVy"
            name-with-owner="Yml0Y29pbi9iaXRjb2lu"
          >

            <template data-target="ref-selector.fetchFailedTemplate">
              <div class="SelectMenu-message" data-index="{{ index }}">Could not load branches</div>
            </template>

              <template data-target="ref-selector.noMatchTemplate">
    <div class="SelectMenu-message">Nothing to show</div>
</template>


            <!-- TODO: this max-height is necessary or else the branch list won't scroll.  why? -->
            <div data-target="ref-selector.listContainer" role="menu" class="SelectMenu-list " style="max-height: 330px">
              <div class="SelectMenu-loading pt-3 pb-0" aria-label="Menu is loading">
                <svg style="box-sizing: content-box; color: var(--color-icon-primary);" viewBox="0 0 16 16" fill="none" data-view-component="true" width="32" height="32" class="anim-rotate">
  <circle cx="8" cy="8" r="7" stroke="currentColor" stroke-opacity="0.25" stroke-width="2" vector-effect="non-scaling-stroke" />
  <path d="M15 8a7.002 7.002 0 00-7-7" stroke="currentColor" stroke-width="2" stroke-linecap="round" vector-effect="non-scaling-stroke" />
</svg>
              </div>
            </div>

              <template data-target="ref-selector.itemTemplate">
  <a href="https://github.com/bitcoin/bitcoin/tree/{{ urlEncodedRefName }}" class="SelectMenu-item" role="menuitemradio" rel="nofollow" aria-checked="{{ isCurrent }}" data-index="{{ index }}">
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-check SelectMenu-icon SelectMenu-icon--check">
    <path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path>
</svg>
    <span class="flex-1 css-truncate css-truncate-overflow {{ isFilteringClass }}">{{ refName }}</span>
    <span hidden="{{ isNotDefault }}" class="Label Label--secondary flex-self-start">default</span>
  </a>
</template>


              <footer class="SelectMenu-footer"><a href="/bitcoin/bitcoin/branches">View all branches</a></footer>
          </ref-selector>

        </div>

        <div role="tabpanel" id="tags-menu" data-filter-placeholder="Find a tag" class="d-flex flex-column flex-auto overflow-auto" tabindex="" hidden>
          <ref-selector
            type="tag"
            data-action="
              input-entered:ref-selector#inputEntered
              tab-selected:ref-selector#tabSelected
              focus-list:ref-selector#focusFirstListMember
            "
            data-targets="input-demux.sinks"
            query-endpoint="/bitcoin/bitcoin/refs"
            cache-key="v0:1622739464.0582771"
            current-committish="bWFzdGVy"
            default-branch="bWFzdGVy"
            name-with-owner="Yml0Y29pbi9iaXRjb2lu"
          >

            <template data-target="ref-selector.fetchFailedTemplate">
              <div class="SelectMenu-message" data-index="{{ index }}">Could not load tags</div>
            </template>

            <template data-target="ref-selector.noMatchTemplate">
              <div class="SelectMenu-message" data-index="{{ index }}">Nothing to show</div>
            </template>

              <template data-target="ref-selector.itemTemplate">
  <a href="https://github.com/bitcoin/bitcoin/tree/{{ urlEncodedRefName }}" class="SelectMenu-item" role="menuitemradio" rel="nofollow" aria-checked="{{ isCurrent }}" data-index="{{ index }}">
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-check SelectMenu-icon SelectMenu-icon--check">
    <path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path>
</svg>
    <span class="flex-1 css-truncate css-truncate-overflow {{ isFilteringClass }}">{{ refName }}</span>
    <span hidden="{{ isNotDefault }}" class="Label Label--secondary flex-self-start">default</span>
  </a>
</template>


            <div data-target="ref-selector.listContainer" role="menu" class="SelectMenu-list" style="max-height: 330px">
              <div class="SelectMenu-loading pt-3 pb-0" aria-label="Menu is loading">
                <svg style="box-sizing: content-box; color: var(--color-icon-primary);" viewBox="0 0 16 16" fill="none" data-view-component="true" width="32" height="32" class="anim-rotate">
  <circle cx="8" cy="8" r="7" stroke="currentColor" stroke-opacity="0.25" stroke-width="2" vector-effect="non-scaling-stroke" />
  <path d="M15 8a7.002 7.002 0 00-7-7" stroke="currentColor" stroke-width="2" stroke-linecap="round" vector-effect="non-scaling-stroke" />
</svg>
              </div>
            </div>
              <footer class="SelectMenu-footer"><a href="/bitcoin/bitcoin/tags">View all tags</a></footer>
          </ref-selector>
        </div>
      </tab-container>
    </input-demux>
  </div>
</div>

  </details>

</div>


  <div class="flex-self-center ml-3 flex-self-stretch d-none d-lg-flex flex-items-center lh-condensed-ultra">
    <a data-pjax href="/bitcoin/bitcoin/branches" class="Link--primary no-underline">
          <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-git-branch text-gray">
    <path fill-rule="evenodd" d="M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z"></path>
</svg>
          <strong>7</strong>
          <span class="color-text-tertiary">branches</span>
    </a>
    <a data-pjax href="/bitcoin/bitcoin/tags" class="ml-3 Link--primary no-underline">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-tag text-gray">
    <path fill-rule="evenodd" d="M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z"></path>
</svg>
        <strong>249</strong>
        <span class="color-text-tertiary">tags</span>
    </a>
  </div>

  <div class="flex-auto"></div>

  <include-fragment data-test-selector="overview-actions-fragment" src="/bitcoin/bitcoin/overview_actions/master"></include-fragment>


    <span class="d-none d-md-flex ml-2">
        
<get-repo>
  <details class="position-relative details-overlay details-reset" data-action="toggle:get-repo#onDetailsToggle">
    <summary class="btn btn-primary" data-hydro-click="{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;repository_id&quot;:1181927,&quot;target&quot;:&quot;CLONE_OR_DOWNLOAD_BUTTON&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="bbdb79cf43ea50486634c8b48b3c6dec3de5e2a011e94b2466373cab3be203fe">
      <svg class="octicon octicon-download mr-1" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.47 10.78a.75.75 0 001.06 0l3.75-3.75a.75.75 0 00-1.06-1.06L8.75 8.44V1.75a.75.75 0 00-1.5 0v6.69L4.78 5.97a.75.75 0 00-1.06 1.06l3.75 3.75zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z"></path></svg>
      Code
      <span class="dropdown-caret"></span>
</summary>    <div class="position-relative">
      <div class="dropdown-menu dropdown-menu-sw p-0" style="top:6px;width:378px;">
          <div data-target="get-repo.modal">
            <div class="border-bottom p-3">
              <a class="Link--muted float-right tooltipped tooltipped-s" href="https://docs.github.com/articles/which-remote-url-should-i-use" target="_blank" aria-label="Which remote URL should I use?">
  <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-question">
    <path fill-rule="evenodd" d="M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004a7.728 7.728 0 00-.313.195 2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.863-1.725A2.76 2.76 0 008 4c-.631 0-1.155.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 101.342.67z"></path>
</svg>
</a>

<div class="text-bold">
  <svg class="octicon octicon-terminal mr-3" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25H1.75zM7.25 8a.75.75 0 01-.22.53l-2.25 2.25a.75.75 0 11-1.06-1.06L5.44 8 3.72 6.28a.75.75 0 111.06-1.06l2.25 2.25c.141.14.22.331.22.53zm1.5 1.5a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z"></path></svg>
  Clone
</div>

<tab-container>

  <div class="UnderlineNav my-2 box-shadow-none">
    <div class="UnderlineNav-body" role="tablist">
          <button name="button" type="button" role="tab" class="UnderlineNav-item lh-default f6 py-0 px-0 mr-2 position-relative" aria-selected="true" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;USE_HTTPS&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="aff095a749269da6c57f6a53edd756af7c8101637235dc8c402443f5979424cb">
            HTTPS
</button>          <button name="button" type="button" role="tab" class="UnderlineNav-item lh-default f6 py-0 px-0 mr-2 position-relative" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;USE_GH_CLI&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="884f12eb9cdb96bbcdc2fd55c68335c4a5e90437235527bdfc4f41c2fb548abe">
            GitHub CLI
</button>    </div>
  </div>

  <div role="tabpanel">
    <div class="input-group">
  <input type="text" class="form-control input-monospace input-sm color-bg-secondary" data-autoselect value="https://github.com/bitcoin/bitcoin.git" aria-label="https://github.com/bitcoin/bitcoin.git" readonly>
  <div class="input-group-button">
    <clipboard-copy value="https://github.com/bitcoin/bitcoin.git" aria-label="Copy to clipboard" class="btn btn-sm js-clipboard-copy tooltipped-no-delay ClipboardButton" data-copy-feedback="Copied!" data-tooltip-direction="n" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;COPY_URL&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="11a44ab28a7c55a003054a9fe2eadae27bc9a9aacfad67333e356bb666c6b5fa"><svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-clippy js-clipboard-clippy-icon d-inline-block">
    <path fill-rule="evenodd" d="M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z"></path>
</svg><svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-check js-clipboard-check-icon color-text-success d-inline-block d-sm-none">
    <path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path>
</svg></clipboard-copy>
  </div>
</div>

    <p class="mt-2 mb-0 f6 color-text-secondary">
        Use Git or checkout with SVN using the web URL.
    </p>
  </div>


  <div role="tabpanel" hidden>
    <div class="input-group">
  <input type="text" class="form-control input-monospace input-sm color-bg-secondary" data-autoselect value="gh repo clone bitcoin/bitcoin" aria-label="gh repo clone bitcoin/bitcoin" readonly>
  <div class="input-group-button">
    <clipboard-copy value="gh repo clone bitcoin/bitcoin" aria-label="Copy to clipboard" class="btn btn-sm js-clipboard-copy tooltipped-no-delay ClipboardButton" data-copy-feedback="Copied!" data-tooltip-direction="n" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;COPY_URL&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="11a44ab28a7c55a003054a9fe2eadae27bc9a9aacfad67333e356bb666c6b5fa"><svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-clippy js-clipboard-clippy-icon d-inline-block">
    <path fill-rule="evenodd" d="M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z"></path>
</svg><svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-check js-clipboard-check-icon color-text-success d-inline-block d-sm-none">
    <path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path>
</svg></clipboard-copy>
  </div>
</div>

    <p class="mt-2 mb-0 f6 color-text-secondary">
      Work fast with our official CLI.
      <a href="https://cli.github.com" target="_blank">Learn more</a>.
    </p>
  </div>
</tab-container>

            </div>
            <ul class="list-style-none">
              <li data-platforms="windows,mac" class="Box-row Box-row--hover-gray p-0 rounded-0 mt-0 js-remove-unless-platform">
                <a class="d-flex flex-items-center color-text-primary text-bold no-underline p-3" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;OPEN_IN_DESKTOP&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="a663945ba8f8553bce164a2727caaac45f338f30584bcf14df89b29e4ae6f928" data-action="click:get-repo#showDownloadMessage" href="https://desktop.github.com">
                  <svg class="octicon octicon-desktop-download mr-3" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path d="M4.927 5.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 5H8.75V.75a.75.75 0 10-1.5 0V5H5.104a.25.25 0 00-.177.427z"></path><path d="M1.573 2.573a.25.25 0 00-.073.177v7.5a.25.25 0 00.25.25h12.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25h-3a.75.75 0 110-1.5h3A1.75 1.75 0 0116 2.75v7.5A1.75 1.75 0 0114.25 12h-3.727c.099 1.041.52 1.872 1.292 2.757A.75.75 0 0111.25 16h-6.5a.75.75 0 01-.565-1.243c.772-.885 1.192-1.716 1.292-2.757H1.75A1.75 1.75 0 010 10.25v-7.5A1.75 1.75 0 011.75 1h3a.75.75 0 010 1.5h-3a.25.25 0 00-.177.073zM6.982 12a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5H6.982z"></path></svg>
                  Open with GitHub Desktop
</a>              </li>
              <li class="Box-row Box-row--hover-gray p-0">
                <a class="d-flex flex-items-center color-text-primary text-bold no-underline p-3" rel="nofollow" data-hydro-click="{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;DOWNLOAD_ZIP&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="dbe8664f4d5f36fc8f0f5e963aea82428cb256241412e67d2585a60c5bd64300" data-ga-click="Repository, download zip, location:repo overview" data-open-app="link" href="/bitcoin/bitcoin/archive/refs/heads/master.zip">
                  <svg class="octicon octicon-file-zip mr-3" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M3.5 1.75a.25.25 0 01.25-.25h3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h2.086a.25.25 0 01.177.073l2.914 2.914a.25.25 0 01.073.177v8.586a.25.25 0 01-.25.25h-.5a.75.75 0 000 1.5h.5A1.75 1.75 0 0014 13.25V4.664c0-.464-.184-.909-.513-1.237L10.573.513A1.75 1.75 0 009.336 0H3.75A1.75 1.75 0 002 1.75v11.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217V1.75zM8.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM6 5.25a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5A.75.75 0 016 5.25zm2 1.5A.75.75 0 018.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 6.75zm-1.25.75a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM8 9.75A.75.75 0 018.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 9.75zm-.75.75a1.75 1.75 0 00-1.75 1.75v3c0 .414.336.75.75.75h2.5a.75.75 0 00.75-.75v-3a1.75 1.75 0 00-1.75-1.75h-.5zM7 12.25a.25.25 0 01.25-.25h.5a.25.25 0 01.25.25v2.25H7v-2.25z"></path></svg>
                  Download ZIP
</a>              </li>
            </ul>
          </div>

        <div class="p-3" data-targets="get-repo.platforms" data-platform="mac" hidden>
          <h4 class="lh-condensed mb-3">Launching GitHub Desktop<span class="AnimatedEllipsis"></span></h4>
          <p class="color-text-secondary">If nothing happens, <a href="https://desktop.github.com/">download GitHub Desktop</a> and try again.</p>
          <button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
        </div>

        <div class="p-3" data-targets="get-repo.platforms" data-platform="windows" hidden>
          <h4 class="lh-condensed mb-3">Launching GitHub Desktop<span class="AnimatedEllipsis"></span></h4>
          <p class="color-text-secondary">If nothing happens, <a href="https://desktop.github.com/">download GitHub Desktop</a> and try again.</p>
          <button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
        </div>

        <div class="p-3" data-targets="get-repo.platforms" data-platform="xcode" hidden>
          <h4 class="lh-condensed mb-3">Launching Xcode<span class="AnimatedEllipsis"></span></h4>
          <p class="color-text-secondary">If nothing happens, <a href="https://developer.apple.com/xcode/">download Xcode</a> and try again.</p>
          <button type="button" class="btn-link" data-action="click:get-repo#onDetailsToggle">Go back</button>
        </div>

        <div class="p-3 " data-targets="get-repo.platforms" data-target="new-codespace.loadingVscode prefetch-pane.loadingVscode" data-platform="vscode" hidden>
  <poll-include-fragment data-target="get-repo.vscodePoller new-codespace.vscodePoller prefetch-pane.vscodePoller">
    <h4 class="lh-condensed mb-3">Launching Visual Studio Code<span class="AnimatedEllipsis" data-hide-on-error></span></h4>
    <p class="color-text-secondary" data-hide-on-error>Your codespace will open once ready.</p>
    <p class="color-text-secondary" data-show-on-error hidden>There was a problem preparing your codespace, please try again.</p>
  </poll-include-fragment>
</div>

      </div>
    </div>
  </details>
</get-repo>


      
    </span>
</div>


      


<div class="Box mb-3">
  <div class="Box-header Box-header--blue position-relative">
    <h2 class="sr-only">Latest commit</h2>
    <div class="js-details-container Details d-flex rounded-top-1 flex-items-center flex-wrap" data-issue-and-pr-hovercards-enabled>
      
  <div class="flex-shrink-0 ml-n1 mr-n1 mt-n1 mb-n1 hx_avatar_stack_commit" >
    
<div class="AvatarStack flex-self-start  " >
  <div class="AvatarStack-body" aria-label="MarcoFalke" >
      <a class="avatar avatar-user" style="width:24px;height:24px;" data-skip-pjax="true" data-test-selector="commits-avatar-stack-avatar-link" data-hovercard-type="user" data-hovercard-url="/users/MarcoFalke/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/MarcoFalke">
        <img data-test-selector="commits-avatar-stack-avatar-image" src="https://avatars.githubusercontent.com/u/6399679?s=48&amp;v=4" width="24" height="24" alt="@MarcoFalke" class=" avatar-user" />
</a>  </div>
</div>

  </div>
  <div class="flex-1 d-flex flex-items-center ml-3 min-width-0">
    <div class="css-truncate css-truncate-overflow color-text-secondary" >
      
      <a href="/bitcoin/bitcoin/commits?author=MarcoFalke"
     class="commit-author user-mention"
     title="View all commits by MarcoFalke">MarcoFalke</a>


  

        <span class="d-none d-sm-inline">
          <a data-pjax="true" data-test-selector="commit-tease-commit-message" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--primary markdown-title" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="789282167" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/20966" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/20966/hovercard" href="https://github.com/bitcoin/bitcoin/pull/20966">#20966</a><a data-pjax="true" data-test-selector="commit-tease-commit-message" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--primary markdown-title" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">: banman: save the banlist in a JSON format on disk</a>
        </span>
    </div>
    <span
      class="hidden-text-expander ml-2 d-inline-block "
      
    >
      <button
        type="button"
        class="color-text-primary ellipsis-expander js-details-target"
        aria-expanded="false"
        
      >
        &hellip;
      </button>
    </span>
    <div class="d-flex flex-auto flex-justify-end ml-3 flex-items-baseline">
        <include-fragment accept="text/fragment+html" src="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d/rollup?direction=sw" class="d-inline"></include-fragment>
      <a
        href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d"
        class="f6 Link--secondary text-mono ml-2 d-none d-lg-inline"
        data-pjax
        
      >
        d6e0d78
      </a>
      <a
        href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d"
        class="Link--secondary ml-2"
        data-pjax
        
      >
        <relative-time datetime="2021-06-23T08:01:56Z" class="no-wrap">Jun 23, 2021</relative-time>
      </a>
    </div>
  </div>
  <div class="pl-0 pl-md-5 flex-order-1 width-full Details-content--hidden">
      <div class="mt-2">
        <a data-pjax="true" data-test-selector="commit-tease-commit-message" class="Link--primary text-bold" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="789282167" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/20966" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/20966/hovercard" href="https://github.com/bitcoin/bitcoin/pull/20966">#20966</a><a data-pjax="true" data-test-selector="commit-tease-commit-message" class="Link--primary text-bold" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">: banman: save the banlist in a JSON format on disk</a>
      </div>
      <pre class="mt-2 text-mono color-text-secondary text-small ws-pre-wrap"><a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard" href="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a"><tt>bb719a0</tt></a> style: remove () from assert in rpc_setban.py (Vasil Dimov)
<a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/24b10ebda301548b8ff4b0c73fefc367ad5dc22b/hovercard" href="https://github.com/bitcoin/bitcoin/commit/24b10ebda301548b8ff4b0c73fefc367ad5dc22b"><tt>24b10eb</tt></a> doc: fix grammar in doc/files.md (Vasil Dimov)
<a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/dd4e957dcdfc971a4a971995ff2db9fb787d23c3/hovercard" href="https://github.com/bitcoin/bitcoin/commit/dd4e957dcdfc971a4a971995ff2db9fb787d23c3"><tt>dd4e957</tt></a> test: ensure banlist can be read from disk after restart (Vasil Dimov)
<a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/d197977ae2076903ed12ab7616a7f93e88be02e1/hovercard" href="https://github.com/bitcoin/bitcoin/commit/d197977ae2076903ed12ab7616a7f93e88be02e1"><tt>d197977</tt></a> banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="783522047" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/20904" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/20904/hovercard" href="https://github.com/bitcoin/bitcoin/pull/20904">#20904</a>
  <span class="issue-keyword tooltipped tooltipped-se" aria-label="This commit closes issue #19748.">Resolves</span> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="680274656" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/19748" data-hovercard-type="issue" data-hovercard-url="/bitcoin/bitcoin/issues/19748/hovercard" href="https://github.com/bitcoin/bitcoin/issues/19748">#19748</a>

ACKs for top commit:
  jonatack:
    Code review re-ACK <a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard" href="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a"><tt>bb719a0</tt></a> per `git range-diff <a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/6a67366fdc3e1d383fe7cbfa209d7e85f0d96638/hovercard" href="https://github.com/bitcoin/bitcoin/commit/6a67366fdc3e1d383fe7cbfa209d7e85f0d96638"><tt>6a67366</tt></a> <a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/4b52c7234f3c2e3067d7d6c6fd7ebf2b96bb8a45/hovercard" href="https://github.com/bitcoin/bitcoin/commit/4b52c7234f3c2e3067d7d6c6fd7ebf2b96bb8a45"><tt>4b52c72</tt></a> <a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard" href="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a"><tt>bb719a0</tt></a>`
  achow101:
    Code Review ACK <a class="commit-link" data-hovercard-type="commit" data-hovercard-url="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard" href="https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a"><tt>bb719a0</tt></a>

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6</pre>
    <div class="d-flex flex-items-center">
      <code class="border d-lg-none mt-2 px-1 rounded-1">d6e0d78</code>
    </div>
  </div>
      <div class="flex-shrink-0">
        <h2 class="sr-only">Git stats</h2>
        <ul class="list-style-none d-flex">
          <li class="ml-0 ml-md-3">
            <a data-pjax href="/bitcoin/bitcoin/commits/master" class="pl-3 pr-3 py-3 p-md-0 mt-n3 mb-n3 mr-n3 m-md-0 Link--primary no-underline no-wrap">
              <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-history text-gray">
    <path fill-rule="evenodd" d="M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z"></path>
</svg>
              <span class="d-none d-sm-inline">
                    <strong>29,619</strong>
                    <span aria-label="Commits on master" class="color-text-secondary d-none d-lg-inline">
                      commits
                    </span>
              </span>
            </a>
          </li>
        </ul>
      </div>
    </div>
  </div>
  <h2 id="files"  class="sr-only">Files</h2>
  


    <a class="d-none js-permalink-shortcut" data-hotkey="y" href="/bitcoin/bitcoin/tree/d6e0d78c31557660274ef53cac912c468eecbe2d">Permalink</a>

  <div data-view-component="true" class="include-fragment-error flash flash-error flash-full py-2">
  <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-alert">
    <path fill-rule="evenodd" d="M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z"></path>
</svg>
  
    Failed to load latest commit information.


  
</div>  <div class="js-details-container Details">
    <div role="grid" aria-labelledby="files" class="Details-content--hidden-not-important js-navigation-container js-active-navigation-container d-md-block" data-pjax>
      <div class="sr-only" role="row">
        <div role="columnheader">Type</div>
        <div role="columnheader">Name</div>
        <div role="columnheader" class="d-none d-md-block">Latest commit message</div>
        <div role="columnheader">Commit time</div>
      </div>

        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".github" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/.github">.github</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="doc: Remove label from good first issue template" class="Link--secondary" href="/bitcoin/bitcoin/commit/fa30d5282cb07b6de0160d7df8b649332db97dde">doc: Remove label from good first issue template</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2020-08-24T07:31:24Z" data-view-component="true" class="no-wrap">Aug 24, 2020</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".tx" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/.tx">.tx</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="qt: Bump transifex slug for 22.x

Opening the 22.x translations early because of experimentation with the
new xliff translations format.

In this context, change file_filter to `xlf` as well as the files
pulled with `tx pull` are that format now (the setting only affects the naming
not the format of the files).

Tree-SHA512: e0c18aa5e6cbd4428d24324fee8e5761b70dae51d0236277577aded719798c6a32fc81c0598f280321f2816629e33a334f61f9e7f6180c4074abfda6550cefbe" class="Link--secondary" href="/bitcoin/bitcoin/commit/417305991a0573484f4aa3820103d8b991cb8f81">qt: Bump transifex slug for 22.x</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-04-21T11:46:41Z" data-view-component="true" class="no-wrap">Apr 21, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="This path skips through empty directories" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/build-aux/m4"><span class="color-text-tertiary">build-aux/</span>m4</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="build, qt: Fix libraries linking order for Linux hosts

This change fixes configuring with Qt on Alpine Linux." class="Link--secondary" href="/bitcoin/bitcoin/commit/a8bd5ea01720e5639cabdc9897cf9cf7c02c47c6">build, qt: Fix libraries linking order for Linux hosts</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-06T20:25:07Z" data-view-component="true" class="no-wrap">Jun 6, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="build_msvc" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/build_msvc">build_msvc</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Merge bitcoin/bitcoin#22230: build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe

9edd713c184bd6b92ff7862d0df8f1a89062e9d3 build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe (Hennadii Stepanov)

Pull request description:

  On master (6f3fbc062f97183f19a8551177371cc74a33351d), running `bitcoin-qt.exe`, which was built with MSVC, causes a terminal window open along with the GUI.

  This PR fixes such behavior. See Microsoft [docs](https://docs.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-160).

  It is still possible to use the `-printtoconsole` option for debug builds.

ACKs for top commit:
  sipsorcery:
    tACK 9edd713c184bd6b92ff7862d0df8f1a89062e9d3.

Tree-SHA512: 02f2874b13e484f98344f6a7e3b01fa82a78a39865787c77bd674ead22a84a7f98a1849ccad26bd2b8c8603b3e29dcc1633b0ad731ce7d61be2d6b1f9584839c" class="Link--secondary" href="/bitcoin/bitcoin/commit/de5512e28df220990ad123b914167aadd6f50979">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="919577035" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/22230" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/22230/hovercard" href="https://github.com/bitcoin/bitcoin/pull/22230">#22230</a><a data-pjax="true" title="Merge bitcoin/bitcoin#22230: build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe

9edd713c184bd6b92ff7862d0df8f1a89062e9d3 build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe (Hennadii Stepanov)

Pull request description:

  On master (6f3fbc062f97183f19a8551177371cc74a33351d), running `bitcoin-qt.exe`, which was built with MSVC, causes a terminal window open along with the GUI.

  This PR fixes such behavior. See Microsoft [docs](https://docs.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-160).

  It is still possible to use the `-printtoconsole` option for debug builds.

ACKs for top commit:
  sipsorcery:
    tACK 9edd713c184bd6b92ff7862d0df8f1a89062e9d3.

Tree-SHA512: 02f2874b13e484f98344f6a7e3b01fa82a78a39865787c77bd674ead22a84a7f98a1849ccad26bd2b8c8603b3e29dcc1633b0ad731ce7d61be2d6b1f9584839c" class="Link--secondary" href="/bitcoin/bitcoin/commit/de5512e28df220990ad123b914167aadd6f50979">: build: Fix MSVC linker /SubSystem option for bitcoin-qt…</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-14T02:06:55Z" data-view-component="true" class="no-wrap">Jun 14, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="ci" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/ci">ci</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="build: enable external signer by default" class="Link--secondary" href="/bitcoin/bitcoin/commit/5be90c907eba0a38019c7d9826623d5d5f567c66">build: enable external signer by default</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-16T08:48:57Z" data-view-component="true" class="no-wrap">Jun 16, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="contrib" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/contrib">contrib</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Merge bitcoin/bitcoin#22244: devtools: Correctly extract symbol versions in symbol-check

e8cd3700eeb27437f5ea435869c9d61214285fdd devtools: Integrate ARCH_MIN_GLIBC_VER table into MAX_VERSIONS in symbol-check.py (W. J. van der Laan)
a33381acf5ae2b43616fffaf26b1c8962e8ef0bb devtools: Add xkb version to symbol-check (W. J. van der Laan)
19e598bab0a1cb5ad93321eb9fa25d1a58d5e276 devtools: Fix verneed section parsing in pixie (W. J. van der Laan)

Pull request description:

  I misunderstood the ELF specification for version symbols (verneed): The `vn_aux` pointer is relative to the main verneed record, not the start of the section.

  This caused many symbols to not be versioned properly in the return value of `elf.dyn_symbols`. This was discovered in #21454.

  Fix it by correcting the offset computation.

  - xkb versions symbols (using the prefix `V`), as this library is used by bitcoin-qt, add it to the valid versions in `symbol-check.py`

  This unfortunately brings to light some symbols that have been introduced since and weren't caught (from a gitian compile of master):

  ```
  bitcoin-cli: symbol getrandom from unsupported version GLIBC_2.25
  bitcoin-cli: failed IMPORTED_SYMBOLS
  bitcoind: symbol getrandom from unsupported version GLIBC_2.25
  bitcoind: symbol log from unsupported version GLIBC_2.29
  bitcoind: symbol fcntl64 from unsupported version GLIBC_2.28
  bitcoind: symbol pow from unsupported version GLIBC_2.29
  bitcoind: symbol exp from unsupported version GLIBC_2.29
  bitcoind: failed IMPORTED_SYMBOLS
  bitcoin-qt: symbol exp from unsupported version GLIBC_2.29
  bitcoin-qt: symbol fcntl64 from unsupported version GLIBC_2.28
  bitcoin-qt: symbol log from unsupported version GLIBC_2.29
  bitcoin-qt: symbol pow from unsupported version GLIBC_2.29
  bitcoin-qt: symbol statx from unsupported version GLIBC_2.28
  bitcoin-qt: symbol getrandom from unsupported version GLIBC_2.25
  bitcoin-qt: symbol renameat2 from unsupported version GLIBC_2.28
  bitcoin-qt: symbol getentropy from unsupported version GLIBC_2.25
  bitcoin-qt: failed IMPORTED_SYMBOLS
  bitcoin-wallet: symbol exp from unsupported version GLIBC_2.29
  bitcoin-wallet: symbol log from unsupported version GLIBC_2.29
  bitcoin-wallet: symbol fcntl64 from unsupported version GLIBC_2.28
  bitcoin-wallet: failed IMPORTED_SYMBOLS
  test_bitcoin: symbol getrandom from unsupported version GLIBC_2.25
  test_bitcoin: symbol log from unsupported version GLIBC_2.29
  test_bitcoin: symbol fcntl64 from unsupported version GLIBC_2.28
  test_bitcoin: symbol pow from unsupported version GLIBC_2.29
  test_bitcoin: symbol exp from unsupported version GLIBC_2.29
  test_bitcoin: failed IMPORTED_SYMBOLS
  ```

ACKs for top commit:
  hebasto:
    ACK e8cd3700eeb27437f5ea435869c9d61214285fdd

Tree-SHA512: 8c15e3478eb642f01a1ddaadef03f80583f088f9fa8e3bf171ce16b0ec05ffb4675ec147d7ffc6a4360637ed47fca517c6ca2bac7bb30d794c03783cfb964b79" class="Link--secondary" href="/bitcoin/bitcoin/commit/a305a687e70cfe1bfe5e57161fa9a084b290cd7b">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="920674557" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/22244" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/22244/hovercard" href="https://github.com/bitcoin/bitcoin/pull/22244">#22244</a><a data-pjax="true" title="Merge bitcoin/bitcoin#22244: devtools: Correctly extract symbol versions in symbol-check

e8cd3700eeb27437f5ea435869c9d61214285fdd devtools: Integrate ARCH_MIN_GLIBC_VER table into MAX_VERSIONS in symbol-check.py (W. J. van der Laan)
a33381acf5ae2b43616fffaf26b1c8962e8ef0bb devtools: Add xkb version to symbol-check (W. J. van der Laan)
19e598bab0a1cb5ad93321eb9fa25d1a58d5e276 devtools: Fix verneed section parsing in pixie (W. J. van der Laan)

Pull request description:

  I misunderstood the ELF specification for version symbols (verneed): The `vn_aux` pointer is relative to the main verneed record, not the start of the section.

  This caused many symbols to not be versioned properly in the return value of `elf.dyn_symbols`. This was discovered in #21454.

  Fix it by correcting the offset computation.

  - xkb versions symbols (using the prefix `V`), as this library is used by bitcoin-qt, add it to the valid versions in `symbol-check.py`

  This unfortunately brings to light some symbols that have been introduced since and weren't caught (from a gitian compile of master):

  ```
  bitcoin-cli: symbol getrandom from unsupported version GLIBC_2.25
  bitcoin-cli: failed IMPORTED_SYMBOLS
  bitcoind: symbol getrandom from unsupported version GLIBC_2.25
  bitcoind: symbol log from unsupported version GLIBC_2.29
  bitcoind: symbol fcntl64 from unsupported version GLIBC_2.28
  bitcoind: symbol pow from unsupported version GLIBC_2.29
  bitcoind: symbol exp from unsupported version GLIBC_2.29
  bitcoind: failed IMPORTED_SYMBOLS
  bitcoin-qt: symbol exp from unsupported version GLIBC_2.29
  bitcoin-qt: symbol fcntl64 from unsupported version GLIBC_2.28
  bitcoin-qt: symbol log from unsupported version GLIBC_2.29
  bitcoin-qt: symbol pow from unsupported version GLIBC_2.29
  bitcoin-qt: symbol statx from unsupported version GLIBC_2.28
  bitcoin-qt: symbol getrandom from unsupported version GLIBC_2.25
  bitcoin-qt: symbol renameat2 from unsupported version GLIBC_2.28
  bitcoin-qt: symbol getentropy from unsupported version GLIBC_2.25
  bitcoin-qt: failed IMPORTED_SYMBOLS
  bitcoin-wallet: symbol exp from unsupported version GLIBC_2.29
  bitcoin-wallet: symbol log from unsupported version GLIBC_2.29
  bitcoin-wallet: symbol fcntl64 from unsupported version GLIBC_2.28
  bitcoin-wallet: failed IMPORTED_SYMBOLS
  test_bitcoin: symbol getrandom from unsupported version GLIBC_2.25
  test_bitcoin: symbol log from unsupported version GLIBC_2.29
  test_bitcoin: symbol fcntl64 from unsupported version GLIBC_2.28
  test_bitcoin: symbol pow from unsupported version GLIBC_2.29
  test_bitcoin: symbol exp from unsupported version GLIBC_2.29
  test_bitcoin: failed IMPORTED_SYMBOLS
  ```

ACKs for top commit:
  hebasto:
    ACK e8cd3700eeb27437f5ea435869c9d61214285fdd

Tree-SHA512: 8c15e3478eb642f01a1ddaadef03f80583f088f9fa8e3bf171ce16b0ec05ffb4675ec147d7ffc6a4360637ed47fca517c6ca2bac7bb30d794c03783cfb964b79" class="Link--secondary" href="/bitcoin/bitcoin/commit/a305a687e70cfe1bfe5e57161fa9a084b290cd7b">: devtools: Correctly extract symbol versions in symbol-c…</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-21T05:58:12Z" data-view-component="true" class="no-wrap">Jun 21, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="depends" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/depends">depends</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="build, qt: Fix compiling qt package in depends with GCC 11" class="Link--secondary" href="/bitcoin/bitcoin/commit/d1d1cc983146ece950430da78ebae6f502913a53">build, qt: Fix compiling qt package in depends with GCC 11</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-08T01:16:36Z" data-view-component="true" class="no-wrap">Jun 8, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="doc" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/doc">doc</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--secondary" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="789282167" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/20966" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/20966/hovercard" href="https://github.com/bitcoin/bitcoin/pull/20966">#20966</a><a data-pjax="true" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--secondary" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">: banman: save the banlist in a JSON format on disk</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-23T08:01:56Z" data-view-component="true" class="no-wrap">Jun 23, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="share" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/share">share</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="doc: add maxuploadtarget to bitcoin.conf example

Introduce the maxuploadtarget option to the example bitcoin.conf file. This adds visibility for this option which is useful to those looking to configure bandwidth usage." class="Link--secondary" href="/bitcoin/bitcoin/commit/947f9734daab4e47c0abdc6ef7d52812102ecb6b">doc: add maxuploadtarget to bitcoin.conf example</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-05-28T16:53:17Z" data-view-component="true" class="no-wrap">May 28, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="src" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/src">src</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--secondary" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="789282167" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/20966" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/20966/hovercard" href="https://github.com/bitcoin/bitcoin/pull/20966">#20966</a><a data-pjax="true" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--secondary" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">: banman: save the banlist in a JSON format on disk</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-23T08:01:56Z" data-view-component="true" class="no-wrap">Jun 23, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="Directory" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file-directory hx_color-icon-directory">
    <path fill-rule="evenodd" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="test" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/tree/master/test">test</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--secondary" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="789282167" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/20966" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/20966/hovercard" href="https://github.com/bitcoin/bitcoin/pull/20966">#20966</a><a data-pjax="true" title="Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk

bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov)
24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov)
dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov)
d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov)

Pull request description:

  Save the banlist in `banlist.json` instead of `banlist.dat`.

  This makes it possible to store Tor v3 entries in the banlist on disk
  (and any other addresses that cannot be serialized in addrv1 format).

  Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade).

  Supersedes https://github.com/bitcoin/bitcoin/pull/20904
  Resolves https://github.com/bitcoin/bitcoin/issues/19748

ACKs for top commit:
  jonatack:
    Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0`
  achow101:
    Code Review ACK bb719a08db173a753984a04534de6f148b87b17a

Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6" class="Link--secondary" href="/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d">: banman: save the banlist in a JSON format on disk</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-23T08:01:56Z" data-view-component="true" class="no-wrap">Jun 23, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".appveyor.yml" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.appveyor.yml">.appveyor.yml</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Switch Appveyor CI to VS2019 stable image

The current appveyor config is using the VS2019 preview image so the latest prebuilt Qt5.12.11 binaries can be used, see #22224.

Appveyor updated the Visual Studio 2019 image to msbuild v16.10.1 on 14th of June. This is the version used to build the latest Qt binaries and removes the need to use the Appveyor VS2019 preview image." class="Link--secondary" href="/bitcoin/bitcoin/commit/aab7fd0f8ddb34437a63d636170f5051aae285b4">Switch Appveyor CI to VS2019 stable image</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-14T19:35:00Z" data-view-component="true" class="no-wrap">Jun 14, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".cirrus.yml" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.cirrus.yml">.cirrus.yml</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="ci: Bump macOS image to big-sur-xcode-12.5

This also removes the &quot;brew update&quot; added in commit
b7381552cd4f965c45f1560d9cfc2fb09dbfcc1d." class="Link--secondary" href="/bitcoin/bitcoin/commit/faa8dfd6a1fcd4df3624aabb3ff08c1f2be198e7">ci: Bump macOS image to big-sur-xcode-12.5</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-02T08:03:38Z" data-view-component="true" class="no-wrap">Jun 2, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".editorconfig" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.editorconfig">.editorconfig</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Add EditorConfig file." class="Link--secondary" href="/bitcoin/bitcoin/commit/7a135d57b2ac17477b25d5046a3bec57eac3ab30">Add EditorConfig file.</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-02-10T07:00:06Z" data-view-component="true" class="no-wrap">Feb 10, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".gitattributes" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.gitattributes">.gitattributes</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Separate protocol versioning from clientversion" class="Link--secondary" href="/bitcoin/bitcoin/commit/71697f97d3f9512f0af934070690c14f1c0d95ea">Separate protocol versioning from clientversion</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2014-10-29T04:24:40Z" data-view-component="true" class="no-wrap">Oct 29, 2014</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".gitignore" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.gitignore">.gitignore</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="build: add *~ to .gitignore

Homebrew autoconf version 2.7.1 introduces configure~ as a build artifact.

Co-authored-by: Hennadii Stepanov &lt;32963518+hebasto@users.noreply.github.com&gt;" class="Link--secondary" href="/bitcoin/bitcoin/commit/bc4538806e3c53e7821e01d5db896f65dd3358ad">build: add *~ to .gitignore</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-05-12T16:10:47Z" data-view-component="true" class="no-wrap">May 12, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".python-version" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.python-version">.python-version</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Bump minimum python version to 3.6" class="Link--secondary" href="/bitcoin/bitcoin/commit/8ae9d314e9af7bcce1e8bc52f0317b9d565109bf">Bump minimum python version to 3.6</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2020-11-09T07:53:47Z" data-view-component="true" class="no-wrap">Nov 9, 2020</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title=".style.yapf" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/.style.yapf">.style.yapf</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="test: .style.yapf: Set column_limit=160" class="Link--secondary" href="/bitcoin/bitcoin/commit/1111f0718acea42954600a4dbd553ac40aae797f">test: .style.yapf: Set column_limit=160</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2019-03-04T23:28:13Z" data-view-component="true" class="no-wrap">Mar 4, 2019</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="CONTRIBUTING.md" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/CONTRIBUTING.md">CONTRIBUTING.md</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="doc: Fix external links (IRC, ...)" class="Link--secondary" href="/bitcoin/bitcoin/commit/9999e4c64b219ffec5105442047daf2b46be0700">doc: Fix external links (IRC, ...)</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-05-31T15:27:57Z" data-view-component="true" class="no-wrap">May 31, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="COPYING" data-pjax="#repo-content-pjax-container" itemprop="license" href="/bitcoin/bitcoin/blob/master/COPYING">COPYING</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="doc: Update license year range to 2021" class="Link--secondary" href="/bitcoin/bitcoin/commit/ccc8d5513fb2228eabc25b105f7a0498f8453885">doc: Update license year range to 2021</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2020-12-30T15:24:47Z" data-view-component="true" class="no-wrap">Dec 30, 2020</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="INSTALL.md" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/INSTALL.md">INSTALL.md</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Update INSTALL landing redirection notice for build instructions." class="Link--secondary" href="/bitcoin/bitcoin/commit/2920be2a6994cfbffd93e72c6cf4c1ed19ac4339">Update INSTALL landing redirection notice for build instructions.</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2016-10-05T23:27:23Z" data-view-component="true" class="no-wrap">Oct 5, 2016</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="Makefile.am" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/Makefile.am">Makefile.am</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Makefile.am: use APP_DIST_DIR instead of hard-coding dist" class="Link--secondary" href="/bitcoin/bitcoin/commit/c090a3e9238ba2df07875b4708e908d8dca4ed9b">Makefile.am: use APP_DIST_DIR instead of hard-coding dist</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-05-13T19:41:56Z" data-view-component="true" class="no-wrap">May 13, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="README.md" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/README.md">README.md</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="doc: Rework internal and external links" class="Link--secondary" href="/bitcoin/bitcoin/commit/77772a1b809e443a6861ee49009ff8bc55cff9c3">doc: Rework internal and external links</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-02-17T08:18:46Z" data-view-component="true" class="no-wrap">Feb 17, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="REVIEWERS" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/REVIEWERS">REVIEWERS</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Update REVIEWERS: I&#39;ve found that I keep track of PRs in need of review without the need for DrahtBot&#39;s automated notification :)" class="Link--secondary" href="/bitcoin/bitcoin/commit/3636d9be8f1daf9160db84c46731b9ef8a7cbc6c">Update REVIEWERS: I've found that I keep track of PRs in need of revi…</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-10T09:00:05Z" data-view-component="true" class="no-wrap">Jun 10, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="SECURITY.md" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/SECURITY.md">SECURITY.md</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="doc: Remove explicit mention of version from SECURITY.md" class="Link--secondary" href="/bitcoin/bitcoin/commit/fa4bc4ebf91e2bf4732063f7a374a98902436a7c">doc: Remove explicit mention of version from SECURITY.md</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2019-06-14T10:39:17Z" data-view-component="true" class="no-wrap">Jun 14, 2019</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="autogen.sh" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/autogen.sh">autogen.sh</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="scripted-diff: Bump copyright of files changed in 2019

-BEGIN VERIFY SCRIPT-
./contrib/devtools/copyright_header.py update ./
-END VERIFY SCRIPT-" class="Link--secondary" href="/bitcoin/bitcoin/commit/aaaaad6ac95b402fe18d019d67897ced6b316ee0">scripted-diff: Bump copyright of files changed in 2019</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2019-12-29T21:42:20Z" data-view-component="true" class="no-wrap">Dec 29, 2019</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="configure.ac" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/configure.ac">configure.ac</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="Merge bitcoin/bitcoin#22238: build: improve detection of eBPF support

8f7704d0321a71c1691837a6bd3b4e05f84d3031 build: improve detection of eBPF support (fanquake)

Pull request description:

  Just checking for the `sys/sdt.h` header isn't enough, as systems like macOS have the header, but it doesn't actually have the `DTRACE_PROBE*` probes, which leads to [compile failures](https://github.com/bitcoin/bitcoin/pull/22006#issuecomment-859559004). The contents of `sys/sdt.h` in the macOS SDK is:
  ```bash
  #ifndef _SYS_SDT_H
  #define _SYS_SDT_H

  /*
   * This is a wrapper header that wraps the mach visible sdt.h header so that
   * the header file ends up visible where software expects it to be.  We also
   * do the C/C++ symbol wrapping here, since Mach headers are technically C
   * interfaces.
   *
   * Note:  The process of adding USDT probes to code is slightly different
   * than documented in the &quot;Solaris Dynamic Tracing Guide&quot;.
   * The DTRACE_PROBE*() macros are not supported on Mac OS X -- instead see
   * &quot;BUILDING CODE CONTAINING USDT PROBES&quot; in the dtrace(1) manpage
   *
   */
  #include &lt;sys/cdefs.h&gt;
  __BEGIN_DECLS
  #include &lt;mach/sdt.h&gt;
  __END_DECLS

  #endif  /* _SYS_SDT_H */
  ```

  The `BUILDING CODE CONTAINING USDT PROBES` section from the dtrace manpage is available [here](https://gist.github.com/fanquake/e56c9866d53b326646d04ab43a8df9e2), and outlines the more involved process of using USDT probes on macOS.

ACKs for top commit:
  jb55:
    utACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031
  practicalswift:
    cr ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031
  hebasto:
    ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031, tested on macOS Big Sur 11.4 (20F71) and on Linux Mint 20.1 (x86_64) with depends.

Tree-SHA512: 5f1351d0ac2e655fccb22a5454f415906404fdaa336fd89b54ef49ca50a442c44ab92d063cba3f161cb8ea0679c92ae3cd6cfbbcb19728cac21116247a017df5" class="Link--secondary" href="/bitcoin/bitcoin/commit/ad0c8f356ee8cc4aad3ff5eef215ffe7420e0ff0">Merge</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="920028295" data-permission-text="Title is private" data-url="https://github.com/bitcoin/bitcoin/issues/22238" data-hovercard-type="pull_request" data-hovercard-url="/bitcoin/bitcoin/pull/22238/hovercard" href="https://github.com/bitcoin/bitcoin/pull/22238">#22238</a><a data-pjax="true" title="Merge bitcoin/bitcoin#22238: build: improve detection of eBPF support

8f7704d0321a71c1691837a6bd3b4e05f84d3031 build: improve detection of eBPF support (fanquake)

Pull request description:

  Just checking for the `sys/sdt.h` header isn't enough, as systems like macOS have the header, but it doesn't actually have the `DTRACE_PROBE*` probes, which leads to [compile failures](https://github.com/bitcoin/bitcoin/pull/22006#issuecomment-859559004). The contents of `sys/sdt.h` in the macOS SDK is:
  ```bash
  #ifndef _SYS_SDT_H
  #define _SYS_SDT_H

  /*
   * This is a wrapper header that wraps the mach visible sdt.h header so that
   * the header file ends up visible where software expects it to be.  We also
   * do the C/C++ symbol wrapping here, since Mach headers are technically C
   * interfaces.
   *
   * Note:  The process of adding USDT probes to code is slightly different
   * than documented in the &quot;Solaris Dynamic Tracing Guide&quot;.
   * The DTRACE_PROBE*() macros are not supported on Mac OS X -- instead see
   * &quot;BUILDING CODE CONTAINING USDT PROBES&quot; in the dtrace(1) manpage
   *
   */
  #include &lt;sys/cdefs.h&gt;
  __BEGIN_DECLS
  #include &lt;mach/sdt.h&gt;
  __END_DECLS

  #endif  /* _SYS_SDT_H */
  ```

  The `BUILDING CODE CONTAINING USDT PROBES` section from the dtrace manpage is available [here](https://gist.github.com/fanquake/e56c9866d53b326646d04ab43a8df9e2), and outlines the more involved process of using USDT probes on macOS.

ACKs for top commit:
  jb55:
    utACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031
  practicalswift:
    cr ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031
  hebasto:
    ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031, tested on macOS Big Sur 11.4 (20F71) and on Linux Mint 20.1 (x86_64) with depends.

Tree-SHA512: 5f1351d0ac2e655fccb22a5454f415906404fdaa336fd89b54ef49ca50a442c44ab92d063cba3f161cb8ea0679c92ae3cd6cfbbcb19728cac21116247a017df5" class="Link--secondary" href="/bitcoin/bitcoin/commit/ad0c8f356ee8cc4aad3ff5eef215ffe7420e0ff0">: build: improve detection of eBPF support</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2021-06-18T07:16:00Z" data-view-component="true" class="no-wrap">Jun 18, 2021</time-ago>
          </div>

        </div>
        <div role="row" class="Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item ">
          <div role="gridcell" class="mr-3 flex-shrink-0" style="width: 16px;">
              <svg aria-label="File" aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-file color-icon-tertiary">
    <path fill-rule="evenodd" d="M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z"></path>
</svg>
          </div>

          <div role="rowheader" class="flex-auto min-width-0 col-md-2 mr-3">
            <span class="css-truncate css-truncate-target d-block width-fit"><a class="js-navigation-open Link--primary" title="libbitcoinconsensus.pc.in" data-pjax="#repo-content-pjax-container" href="/bitcoin/bitcoin/blob/master/libbitcoinconsensus.pc.in">libbitcoinconsensus.pc.in</a></span>
          </div>

          <div role="gridcell" class="flex-auto min-width-0 d-none d-md-block col-5 mr-3" >
              <span class="css-truncate css-truncate-target d-block width-fit markdown-title">
                    <a data-pjax="true" title="build: remove libcrypto as internal dependency in libbitcoinconsensus.pc" class="Link--secondary" href="/bitcoin/bitcoin/commit/2d7066527a456f8e1f4f603fe104b0bd9d864559">build: remove libcrypto as internal dependency in libbitcoinconsensus.pc</a>
              </span>
          </div>

          <div role="gridcell" class="color-text-tertiary text-right" style="width:100px;">
              <time-ago datetime="2019-11-19T14:03:44Z" data-view-component="true" class="no-wrap">Nov 19, 2019</time-ago>
          </div>

        </div>
    </div>
    <div class="Details-content--shown Box-footer d-md-none p-0">
      <button type="button" class="d-block btn-link js-details-target width-full px-3 py-2" aria-expanded="false">
        View code
      </button>
    </div>
  </div>




</div>

    <readme-toc>

    <div id="readme" class="Box md js-code-block-container Box--responsive">

      <div class="d-flex  js-sticky js-position-sticky top-0 border-top-0 border-bottom p-2 flex-items-center flex-justify-between color-bg-primary rounded-top-2"  style="position: sticky; z-index: 90;" >
        <div class="d-flex flex-items-center">
            <details
  data-target="readme-toc.trigger"
  data-menu-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;trigger&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}"
  data-menu-hydro-click-hmac="6efdafc4692df2788cc9474cb391c7072addd46122e74942558b7ab2f8d1fce1"
  class="dropdown details-reset details-overlay"
>
  <summary
    class="btn btn-octicon m-0 mr-2 p-2"
    aria-haspopup="true"
    aria-label="Table of Contents">
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-list-unordered">
    <path fill-rule="evenodd" d="M2 4a1 1 0 100-2 1 1 0 000 2zm3.75-1.5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z"></path>
</svg>
  </summary>

  <details-menu class="SelectMenu" role="menu">
    <div class="SelectMenu-modal rounded-3 mt-1" style="max-height:340px;">
      <div class="SelectMenu-list SelectMenu-list--borderless p-2" style="overscroll-behavior: contain;">

          <a role="menuitem" class="filter-item py-1 text-emphasized" style="padding-left: 12px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#bitcoin-core-integrationstaging-tree">Bitcoin Core integration/staging tree</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 24px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#what-is-bitcoin">What is Bitcoin?</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 24px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#license">License</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 24px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#development-process">Development Process</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 24px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#testing">Testing</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 36px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#automated-testing">Automated Testing</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 36px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#manual-quality-assurance-qa-testing">Manual Quality Assurance (QA) Testing</a>
          <a role="menuitem" class="filter-item py-1 " style="padding-left: 24px;" data-action="click:readme-toc#blur" data-targets="readme-toc.entries" data-hydro-click="{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}" data-hydro-click-hmac="affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625" href="#translations">Translations</a>
      </div>
    </div>
  </details-menu>
</details>

          <h2 class="Box-title">
            <a href="#readme" data-view-component="true" class="Link--primary">README.md</a>
          </h2>
        </div>
      </div>

          <div class="Popover anim-scale-in js-tagsearch-popover"
     hidden
     data-tagsearch-url="/bitcoin/bitcoin/find-definition"
     data-tagsearch-ref="master"
     data-tagsearch-path="README.md"
     data-tagsearch-lang="Markdown"
     data-hydro-click="{&quot;event_type&quot;:&quot;code_navigation.click_on_symbol&quot;,&quot;payload&quot;:{&quot;action&quot;:&quot;click_on_symbol&quot;,&quot;repository_id&quot;:1181927,&quot;ref&quot;:&quot;master&quot;,&quot;language&quot;:&quot;Markdown&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}"
     data-hydro-click-hmac="7cb0392b00d603ce84bc4959068c92914f7b08e0dff712cd2e70475f64a0ce8c">
  <div class="Popover-message Popover-message--large Popover-message--top-left TagsearchPopover mt-1 mb-4 mx-auto Box color-shadow-large">
    <div class="TagsearchPopover-content js-tagsearch-popover-content overflow-auto" style="will-change:transform;">
    </div>
  </div>
</div>

        <div data-target="readme-toc.content" class="Box-body px-5 pb-5">
          <article class="markdown-body entry-content container-lg" itemprop="text"><h1><a id="user-content-bitcoin-core-integrationstaging-tree" class="anchor" aria-hidden="true" href="#bitcoin-core-integrationstaging-tree"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Bitcoin Core integration/staging tree</h1>
<p><a href="https://bitcoincore.org" rel="nofollow">https://bitcoincore.org</a></p>
<p>For an immediately usable, binary version of the Bitcoin Core software, see
<a href="https://bitcoincore.org/en/download/" rel="nofollow">https://bitcoincore.org/en/download/</a>.</p>
<p>Further information about Bitcoin Core is available in the <a href="/bitcoin/bitcoin/blob/master/doc">doc folder</a>.</p>
<h2><a id="user-content-what-is-bitcoin" class="anchor" aria-hidden="true" href="#what-is-bitcoin"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>What is Bitcoin?</h2>
<p>Bitcoin is an experimental digital currency that enables instant payments to
anyone, anywhere in the world. Bitcoin uses peer-to-peer technology to operate
with no central authority: managing transactions and issuing money are carried
out collectively by the network. Bitcoin Core is the name of open source
software which enables the use of this currency.</p>
<p>For more information read the original Bitcoin whitepaper.</p>
<h2><a id="user-content-license" class="anchor" aria-hidden="true" href="#license"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>License</h2>
<p>Bitcoin Core is released under the terms of the MIT license. See <a href="/bitcoin/bitcoin/blob/master/COPYING">COPYING</a> for more
information or see <a href="https://opensource.org/licenses/MIT" rel="nofollow">https://opensource.org/licenses/MIT</a>.</p>
<h2><a id="user-content-development-process" class="anchor" aria-hidden="true" href="#development-process"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Development Process</h2>
<p>The <code>master</code> branch is regularly built (see <code>doc/build-*.md</code> for instructions) and tested, but it is not guaranteed to be
completely stable. <a href="https://github.com/bitcoin/bitcoin/tags">Tags</a> are created
regularly from release branches to indicate new official, stable release versions of Bitcoin Core.</p>
<p>The <a href="https://github.com/bitcoin-core/gui">https://github.com/bitcoin-core/gui</a> repository is used exclusively for the
development of the GUI. Its master branch is identical in all monotree
repositories. Release branches and tags do not exist, so please do not fork
that repository unless it is for development reasons.</p>
<p>The contribution workflow is described in <a href="/bitcoin/bitcoin/blob/master/CONTRIBUTING.md">CONTRIBUTING.md</a>
and useful hints for developers can be found in <a href="/bitcoin/bitcoin/blob/master/doc/developer-notes.md">doc/developer-notes.md</a>.</p>
<h2><a id="user-content-testing" class="anchor" aria-hidden="true" href="#testing"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Testing</h2>
<p>Testing and code review is the bottleneck for development; we get more pull
requests than we can review and test on short notice. Please be patient and help out by testing
other people's pull requests, and remember this is a security-critical project where any mistake might cost people
lots of money.</p>
<h3><a id="user-content-automated-testing" class="anchor" aria-hidden="true" href="#automated-testing"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Automated Testing</h3>
<p>Developers are strongly encouraged to write <a href="/bitcoin/bitcoin/blob/master/src/test/README.md">unit tests</a> for new code, and to
submit new unit tests for old code. Unit tests can be compiled and run
(assuming they weren't disabled in configure) with: <code>make check</code>. Further details on running
and extending unit tests can be found in <a href="/bitcoin/bitcoin/blob/master/src/test/README.md">/src/test/README.md</a>.</p>
<p>There are also <a href="/bitcoin/bitcoin/blob/master/test">regression and integration tests</a>, written
in Python.
These tests can be run (if the <a href="/bitcoin/bitcoin/blob/master/test">test dependencies</a> are installed) with: <code>test/functional/test_runner.py</code></p>
<p>The CI (Continuous Integration) systems make sure that every pull request is built for Windows, Linux, and macOS,
and that unit/sanity tests are run automatically.</p>
<h3><a id="user-content-manual-quality-assurance-qa-testing" class="anchor" aria-hidden="true" href="#manual-quality-assurance-qa-testing"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Manual Quality Assurance (QA) Testing</h3>
<p>Changes should be tested by somebody other than the developer who wrote the
code. This is especially important for large or high-risk changes. It is useful
to add a test plan to the pull request description if testing the changes is
not straightforward.</p>
<h2><a id="user-content-translations" class="anchor" aria-hidden="true" href="#translations"><svg class="octicon octicon-link" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path></svg></a>Translations</h2>
<p>Changes to translations as well as new translations can be submitted to
<a href="https://www.transifex.com/bitcoin/bitcoin/" rel="nofollow">Bitcoin Core's Transifex page</a>.</p>
<p>Translations are periodically pulled from Transifex and merged into the git repository. See the
<a href="/bitcoin/bitcoin/blob/master/doc/translation_process.md">translation process</a> for details on how this works.</p>
<p><strong>Important</strong>: We do not accept translation changes as GitHub pull requests because the next
pull from Transifex would automatically overwrite them again.</p>
</article>
        </div>
    </div>

  </readme-toc>


</div>

    <div data-view-component="true" class="flex-shrink-0 col-12 col-md-3">      

      <div class="BorderGrid BorderGrid--spacious" data-pjax>
        <div class="BorderGrid-row hide-sm hide-md">
          <div class="BorderGrid-cell">
            <h2 class="mb-3 h4">About</h2>

    <p class="f4 mt-3">
      Bitcoin Core integration/staging tree
    </p>
    <div class="mt-3 d-flex flex-items-center">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-link flex-shrink-0 mr-2">
    <path fill-rule="evenodd" d="M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z"></path>
</svg>
      <span class="flex-auto min-width-0 css-truncate css-truncate-target width-fit">
        <a title="https://bitcoincore.org/en/download" role="link" target="_blank" class="text-bold" rel="noopener noreferrer" href="https://bitcoincore.org/en/download">bitcoincore.org/en/download</a>
      </span>
    </div>

  <h3 class="sr-only">Topics</h3>
  <div class="mt-3">
      <div class="f6">
      <a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:c-plus-plus" href="/topics/c-plus-plus" title="Topic: c-plus-plus" data-view-component="true" class="topic-tag topic-tag-link">
  c-plus-plus
</a>
      <a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:cryptography" href="/topics/cryptography" title="Topic: cryptography" data-view-component="true" class="topic-tag topic-tag-link">
  cryptography
</a>
      <a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:bitcoin" href="/topics/bitcoin" title="Topic: bitcoin" data-view-component="true" class="topic-tag topic-tag-link">
  bitcoin
</a>
      <a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:p2p" href="/topics/p2p" title="Topic: p2p" data-view-component="true" class="topic-tag topic-tag-link">
  p2p
</a>
      <a data-ga-click="Topic, repository page" data-octo-click="topic_click" data-octo-dimensions="topic:cryptocurrency" href="/topics/cryptocurrency" title="Topic: cryptocurrency" data-view-component="true" class="topic-tag topic-tag-link">
  cryptocurrency
</a>
  </div>

  </div>

  <h3 class="sr-only">Resources</h3>
  <div class="mt-3">
    <a class="Link--muted" href="#readme">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-book mr-2">
    <path fill-rule="evenodd" d="M0 1.75A.75.75 0 01.75 1h4.253c1.227 0 2.317.59 3 1.501A3.744 3.744 0 0111.006 1h4.245a.75.75 0 01.75.75v10.5a.75.75 0 01-.75.75h-4.507a2.25 2.25 0 00-1.591.659l-.622.621a.75.75 0 01-1.06 0l-.622-.621A2.25 2.25 0 005.258 13H.75a.75.75 0 01-.75-.75V1.75zm8.755 3a2.25 2.25 0 012.25-2.25H14.5v9h-3.757c-.71 0-1.4.201-1.992.572l.004-7.322zm-1.504 7.324l.004-5.073-.002-2.253A2.25 2.25 0 005.003 2.5H1.5v9h3.757a3.75 3.75 0 011.994.574z"></path>
</svg>
      Readme
</a>  </div>

  <h3 class="sr-only">License</h3>
  <div class="mt-3">
    <a href="/bitcoin/bitcoin/blob/master/COPYING" class="Link--muted" >
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-law mr-2">
    <path fill-rule="evenodd" d="M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z"></path>
</svg>
        MIT License
    </a>
  </div>

          </div>
        </div>
          <div class="BorderGrid-row">
            <div class="BorderGrid-cell">
              <h2 class="h4 mb-3">
  <a href="/bitcoin/bitcoin/releases" data-view-component="true" class="Link--primary no-underline">
    Releases
      <span title="249" data-view-component="true" class="Counter">249</span>
</a></h2>

  <a class="Link--primary d-flex no-underline" href="/bitcoin/bitcoin/releases/tag/v0.21.1">
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-tag flex-shrink-0 mt-1 color-text-success">
    <path fill-rule="evenodd" d="M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z"></path>
</svg>
    <div class="ml-2 min-width-0">
      <div class="d-flex">
        <span class="css-truncate css-truncate-target text-bold mr-2" style="max-width: none;">Bitcoin Core 0.21.1</span>
        <span title="Label: Latest" data-view-component="true" class="Label Label--success flex-shrink-0">
          Latest
</span>      </div>
      <div class="text-small color-text-secondary"><relative-time datetime="2021-05-03T01:33:04Z" class="no-wrap">May 3, 2021</relative-time></div>
    </div>
</a>    <div data-view-component="true" class="mt-3">
      <a href="/bitcoin/bitcoin/releases" data-view-component="true" class="text-small">
        + 248 releases
</a></div>
            </div>
          </div>
          <div class="BorderGrid-row">
            <div class="BorderGrid-cell">
              <h2 class="h4 mb-3">
  <a href="/orgs/bitcoin/packages?repo_name=bitcoin" data-view-component="true" class="Link--primary no-underline">
    Packages <span title="0" hidden="hidden" data-view-component="true" class="Counter">0</span>
</a></h2>


      <div class="text-small color-text-secondary">
        No packages published <br>
      </div>



            </div>
          </div>
          <div class="BorderGrid-row" hidden>
            <div class="BorderGrid-cell">
              <include-fragment src="/bitcoin/bitcoin/used_by_list" accept="text/fragment+html">
</include-fragment>
            </div>
          </div>
          <div class="BorderGrid-row">
            <div class="BorderGrid-cell">
              <h2 class="h4 mb-3">
  <a href="/bitcoin/bitcoin/graphs/contributors" data-view-component="true" class="Link--primary no-underline">
    Contributors <span title="802" data-view-component="true" class="Counter">802</span>
</a></h2>


    
  <ul class="list-style-none d-flex flex-wrap mb-n2">
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/laanwj/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/laanwj">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/126646?s=64&amp;v=4" width="32" height="32" alt="@laanwj" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/MarcoFalke/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/MarcoFalke">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/6399679?s=64&amp;v=4" width="32" height="32" alt="@MarcoFalke" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/sipa/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/sipa">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/548488?s=64&amp;v=4" width="32" height="32" alt="@sipa" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/fanquake/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/fanquake">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/863730?s=64&amp;v=4" width="32" height="32" alt="@fanquake" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/gavinandresen/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/gavinandresen">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/331997?s=64&amp;v=4" width="32" height="32" alt="@gavinandresen" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/jonasschnelli/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/jonasschnelli">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/178464?s=64&amp;v=4" width="32" height="32" alt="@jonasschnelli" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/hebasto/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/hebasto">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/32963518?s=64&amp;v=4" width="32" height="32" alt="@hebasto" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/practicalswift/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/practicalswift">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/7826565?s=64&amp;v=4" width="32" height="32" alt="@practicalswift" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/jnewbery/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/jnewbery">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/1063656?s=64&amp;v=4" width="32" height="32" alt="@jnewbery" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/TheBlueMatt/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/TheBlueMatt">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/649246?s=64&amp;v=4" width="32" height="32" alt="@TheBlueMatt" />
</a>      </li>
      <li class="mb-2 mr-2">
        <a class="" data-hovercard-type="user" data-hovercard-url="/users/theuni/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="/theuni">
          <img class="d-block avatar-user" src="https://avatars.githubusercontent.com/u/417043?s=64&amp;v=4" width="32" height="32" alt="@theuni" />
</a>      </li>
  </ul>


  <div data-view-component="true" class="mt-3">
    <a href="/bitcoin/bitcoin/graphs/contributors" data-view-component="true" class="text-small">
      + 791 contributors
</a></div>
            </div>
          </div>
          <div class="BorderGrid-row">
            <div class="BorderGrid-cell">
              <h2 class="h4 mb-3">Languages</h2>
<div class="mb-2">
  <span data-view-component="true" class="Progress">
    <span itemprop="keywords" aria-label="C++ 66.6" style="background-color: #f34b7d;width: 66.6%;" data-view-component="true" class="Progress-item"></span>
    <span itemprop="keywords" aria-label="Python 18.9" style="background-color: #3572A5;width: 18.9%;" data-view-component="true" class="Progress-item"></span>
    <span itemprop="keywords" aria-label="C 9.2" style="background-color: #555555;width: 9.2%;" data-view-component="true" class="Progress-item"></span>
    <span itemprop="keywords" aria-label="M4 1.6" style="background-color: #ccc;width: 1.6%;" data-view-component="true" class="Progress-item"></span>
    <span itemprop="keywords" aria-label="Shell 1.6" style="background-color: #89e051;width: 1.6%;" data-view-component="true" class="Progress-item"></span>
    <span itemprop="keywords" aria-label="Makefile 1.0" style="background-color: #427819;width: 1.0%;" data-view-component="true" class="Progress-item"></span>
    <span itemprop="keywords" aria-label="Other 1.1" style="background-color: #ededed;width: 1.1%;" data-view-component="true" class="Progress-item"></span>
</span></div>
<ul class="list-style-none">
    <li class="d-inline">
      <a class="d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3" href="/bitcoin/bitcoin/search?l=c%2B%2B"  data-ga-click="Repository, language stats search click, location:repo overview">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#f34b7d;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">C++</span>
        <span>66.6%</span>
      </a>
    </li>
    <li class="d-inline">
      <a class="d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3" href="/bitcoin/bitcoin/search?l=python"  data-ga-click="Repository, language stats search click, location:repo overview">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#3572A5;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">Python</span>
        <span>18.9%</span>
      </a>
    </li>
    <li class="d-inline">
      <a class="d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3" href="/bitcoin/bitcoin/search?l=c"  data-ga-click="Repository, language stats search click, location:repo overview">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#555555;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">C</span>
        <span>9.2%</span>
      </a>
    </li>
    <li class="d-inline">
      <a class="d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3" href="/bitcoin/bitcoin/search?l=m4"  data-ga-click="Repository, language stats search click, location:repo overview">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#ccc;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">M4</span>
        <span>1.6%</span>
      </a>
    </li>
    <li class="d-inline">
      <a class="d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3" href="/bitcoin/bitcoin/search?l=shell"  data-ga-click="Repository, language stats search click, location:repo overview">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#89e051;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">Shell</span>
        <span>1.6%</span>
      </a>
    </li>
    <li class="d-inline">
      <a class="d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3" href="/bitcoin/bitcoin/search?l=makefile"  data-ga-click="Repository, language stats search click, location:repo overview">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#427819;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">Makefile</span>
        <span>1.0%</span>
      </a>
    </li>
    <li class="d-inline">
      <span class="d-inline-flex flex-items-center flex-nowrap text-small mr-3">
        <svg class="octicon octicon-dot-fill mr-2" style="color:#ededed;" viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true"><path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8z"></path></svg>
        <span class="color-text-primary text-bold mr-1">Other</span>
        <span>1.1%</span>
      </span>
    </li>
</ul>

            </div>
          </div>
      </div>
</div>
</div></div>



  </div>
</div>

    </main>
  </div>

  </div>

          
<div class="footer container-xl width-full p-responsive" role="contentinfo">
  <div class="position-relative d-flex flex-row-reverse flex-lg-row flex-wrap flex-lg-nowrap flex-justify-center flex-lg-justify-between pt-6 pb-2 mt-6 f6 color-text-secondary border-top color-border-secondary ">
    <ul class="list-style-none d-flex flex-wrap col-12 col-lg-5 flex-justify-center flex-lg-justify-between mb-2 mb-lg-0">
      <li class="mr-3 mr-lg-0">&copy; 2021 GitHub, Inc.</li>
        <li class="mr-3 mr-lg-0"><a href="https://docs.github.com/en/github/site-policy/github-terms-of-service" data-ga-click="Footer, go to terms, text:terms">Terms</a></li>
        <li class="mr-3 mr-lg-0"><a href="https://docs.github.com/en/github/site-policy/github-privacy-statement" data-ga-click="Footer, go to privacy, text:privacy">Privacy</a></li>
        <li class="mr-3 mr-lg-0"><a data-ga-click="Footer, go to security, text:security" href="https://github.com/security">Security</a></li>
        <li class="mr-3 mr-lg-0"><a href="https://www.githubstatus.com/" data-ga-click="Footer, go to status, text:status">Status</a></li>
        <li><a data-ga-click="Footer, go to help, text:Docs" href="https://docs.github.com">Docs</a></li>
    </ul>

    <a aria-label="Homepage" title="GitHub" class="footer-octicon d-none d-lg-block mx-lg-4" href="https://github.com">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="24" width="24" class="octicon octicon-mark-github">
    <path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
</svg>
</a>
    <ul class="list-style-none d-flex flex-wrap col-12 col-lg-5 flex-justify-center flex-lg-justify-between mb-2 mb-lg-0">
        <li class="mr-3 mr-lg-0"><a href="https://support.github.com" data-ga-click="Footer, go to contact, text:contact">Contact GitHub</a></li>
        <li class="mr-3 mr-lg-0"><a href="https://github.com/pricing" data-ga-click="Footer, go to Pricing, text:Pricing">Pricing</a></li>
      <li class="mr-3 mr-lg-0"><a href="https://docs.github.com" data-ga-click="Footer, go to api, text:api">API</a></li>
      <li class="mr-3 mr-lg-0"><a href="https://services.github.com" data-ga-click="Footer, go to training, text:training">Training</a></li>
        <li class="mr-3 mr-lg-0"><a href="https://github.blog" data-ga-click="Footer, go to blog, text:blog">Blog</a></li>
        <li><a data-ga-click="Footer, go to about, text:about" href="https://github.com/about">About</a></li>
    </ul>
  </div>
  <div class="d-flex flex-justify-center pb-6">
    <span class="f6 color-text-tertiary"></span>
  </div>

  
</div>



  <div id="ajax-error-message" class="ajax-error-message flash flash-error" hidden>
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-alert">
    <path fill-rule="evenodd" d="M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z"></path>
</svg>
    <button type="button" class="flash-close js-ajax-error-dismiss" aria-label="Dismiss error">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-x">
    <path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path>
</svg>
    </button>
    You can’t perform that action at this time.
  </div>

  <div class="js-stale-session-flash flash flash-warn flash-banner" hidden
    >
    <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-alert">
    <path fill-rule="evenodd" d="M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z"></path>
</svg>
    <span class="js-stale-session-flash-signed-in" hidden>You signed in with another tab or window. <a href="">Reload</a> to refresh your session.</span>
    <span class="js-stale-session-flash-signed-out" hidden>You signed out in another tab or window. <a href="">Reload</a> to refresh your session.</span>
  </div>
    <template id="site-details-dialog">
  <details class="details-reset details-overlay details-overlay-dark lh-default color-text-primary hx_rsm" open>
    <summary role="button" aria-label="Close dialog"></summary>
    <details-dialog class="Box Box--overlay d-flex flex-column anim-fade-in fast hx_rsm-dialog hx_rsm-modal">
      <button class="Box-btn-octicon m-0 btn-octicon position-absolute right-0 top-0" type="button" aria-label="Close dialog" data-close-dialog>
        <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-x">
    <path fill-rule="evenodd" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"></path>
</svg>
      </button>
      <div class="octocat-spinner my-6 js-details-dialog-spinner"></div>
    </details-dialog>
  </details>
</template>

    <div class="Popover js-hovercard-content position-absolute" style="display: none; outline: none;" tabindex="0">
  <div class="Popover-message Popover-message--bottom-left Popover-message--large Box color-shadow-large" style="width:360px;">
  </div>
</div>

    <template id="snippet-clipboard-copy-button">
  <div class="zeroclipboard-container position-absolute right-0 top-0">
    <clipboard-copy aria-label="Copy" class="ClipboardButton btn js-clipboard-copy m-2 p-0 tooltipped-no-delay" data-copy-feedback="Copied!" data-tooltip-direction="w">
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-clippy js-clipboard-clippy-icon m-2">
    <path fill-rule="evenodd" d="M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z"></path>
</svg>
      <svg aria-hidden="true" viewBox="0 0 16 16" version="1.1" data-view-component="true" height="16" width="16" class="octicon octicon-check js-clipboard-check-icon color-text-success d-none m-2">
    <path fill-rule="evenodd" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"></path>
</svg>
    </clipboard-copy>
  </div>
</template>



  

  </body>
</html>

<meta property="og:image" content="https://opengraph.githubassets.com/d1bafea84bf4320a8c28a296cb97c75a7af04cc4dece4fea51a6c80889bd7ee1/bitcoin/bitcoin" />

Response(url: Optional(https://github.com/bitcoin/bitcoin), finalUrl: Optional(https://github.com/bitcoin/bitcoin), canonicalUrl: Optional("github.com"), title: Optional("bitcoin/bitcoin"), description: Optional("Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub."), images: Optional(["https://github.githubassets.com/images/search-key-slash.svg", "https://github.com/notifications/beta/shelf", "https://avatars.githubusercontent.com/u/6399679", "https://avatars.githubusercontent.com/u/126646", "https://avatars.githubusercontent.com/u/6399679", "https://avatars.githubusercontent.com/u/548488", "https://avatars.githubusercontent.com/u/863730", "https://avatars.githubusercontent.com/u/331997", "https://avatars.githubusercontent.com/u/178464", "https://avatars.githubusercontent.com/u/32963518", "https://avatars.githubusercontent.com/u/7826565", "https://avatars.githubusercontent.com/u/1063656", "https://avatars.githubusercontent.com/u/649246", "https://avatars.githubusercontent.com/u/417043"]), image: Optional("https://github.githubassets.com/images/search-key-slash.svg"), icon: Optional("https://github.com/fluidicon.png"), video: nil, price: nil)


"<!DOCTYPE html> <html lang=\"en\" data-color-mode=\"auto\" data-light-theme=\"light\" data-dark-theme=\"dark\"> <head> <meta charset=\"utf-8\"> <meta name=\"viewport\" content=\"width=device-width\"> <title>GitHub - bitcoin/bitcoin: Bitcoin Core integration/staging tree</title> <meta name=\"description\" content=\"Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub.\"> <meta property=\"fb:app_id\" content=\"1401488693436528\"> <meta name=\"apple-itunes-app\" content=\"app-id=1477376905\" /> <meta name=\"twitter:image:src\" content=\"https://opengraph.githubassets.com/73871cbf45a7f42243be226ba8252210600ba29af1a78121a64a89beeb184065/bitcoin/bitcoin\" /><meta name=\"twitter:site\" content=\"@github\" /><meta name=\"twitter:card\" content=\"summary_large_image\" /><meta name=\"twitter:title\" content=\"bitcoin/bitcoin\" /><meta name=\"twitter:description\" content=\"Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub.\" /> <meta property=\"og:image\" content=\"https://opengraph.githubassets.com/73871cbf45a7f42243be226ba8252210600ba29af1a78121a64a89beeb184065/bitcoin/bitcoin\" /><meta property=\"og:image:alt\" content=\"Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub.\" /><meta property=\"og:image:width\" content=\"1200\" /><meta property=\"og:image:height\" content=\"600\" /><meta property=\"og:site_name\" content=\"GitHub\" /><meta property=\"og:type\" content=\"object\" /><meta property=\"og:title\" content=\"bitcoin/bitcoin\" /><meta property=\"og:url\" content=\"https://github.com/bitcoin/bitcoin\" /><meta property=\"og:description\" content=\"Bitcoin Core integration/staging tree. Contribute to bitcoin/bitcoin development by creating an account on GitHub.\" /> <meta name=\"request-id\" content=\"E6AE:0E74:39A87B:3F89B4:60D302CD\" data-pjax-transient=\"true\"/><meta name=\"html-safe-nonce\" content=\"5d0eec287c5eb4744a80e33ab6f8d57e661da56dd53bb0f3f6ca4893664a6947\" data-pjax-transient=\"true\"/><meta name=\"visitor-payload\" content=\"eyJyZWZlcnJlciI6IiIsInJlcXVlc3RfaWQiOiJFNkFFOjBFNzQ6MzlBODdCOjNGODlCNDo2MEQzMDJDRCIsInZpc2l0b3JfaWQiOiIxODc5NTUwMDk5MjA3NjA2ODg5IiwicmVnaW9uX2VkZ2UiOiJhcC1zb3V0aGVhc3QtMSIsInJlZ2lvbl9yZW5kZXIiOiJhcC1zb3V0aGVhc3QtMSJ9\" data-pjax-transient=\"true\"/><meta name=\"visitor-hmac\" content=\"ad52938aa0acf4c1e5adf43a2b6acfefe4eb44fadcee39af564a0964558fd68c\" data-pjax-transient=\"true\"/> <meta name=\"hovercard-subject-tag\" content=\"repository:1181927\" data-pjax-transient> <meta name=\"github-keyboard-shortcuts\" content=\"repository\" data-pjax-transient=\"true\" /> <meta name=\"selected-link\" value=\"repo_source\" data-pjax-transient> <meta name=\"google-site-verification\" content=\"c1kuD-K2HIVF635lypcsWPoD4kilo5-jA_wBFyT4uMY\"> <meta name=\"google-site-verification\" content=\"KT5gs8h0wvaagLKAVWq8bbeNwnZZK1r1XQysX3xurLU\"> <meta name=\"google-site-verification\" content=\"ZzhVyEFwb7w3e0-uOTltm8Jsck2F5StVihD0exw2fsA\"> <meta name=\"google-site-verification\" content=\"GXs5KoUUkNCoaAZn7wPN-t01Pywp9M3sEjnt_3_ZWPc\"> <meta name=\"octolytics-host\" content=\"collector.githubapp.com\" /><meta name=\"octolytics-app-id\" content=\"github\" /><meta name=\"octolytics-event-url\" content=\"https://collector.githubapp.com/github-external/browser_event\" /> <meta name=\"analytics-location\" content=\"/&lt;user-name&gt;/&lt;repo-name&gt;\" data-pjax-transient=\"true\" /> <meta name=\"optimizely-datafile\" content=\"{&quot;version&quot;: &quot;4&quot;, &quot;rollouts&quot;: [], &quot;typedAudiences&quot;: [], &quot;anonymizeIP&quot;: true, &quot;projectId&quot;: &quot;16737760170&quot;, &quot;variables&quot;: [], &quot;featureFlags&quot;: [], &quot;experiments&quot;: [{&quot;status&quot;: &quot;Running&quot;, &quot;audienceIds&quot;: [], &quot;variations&quot;: [{&quot;variables&quot;: [], &quot;id&quot;: &quot;20227754799&quot;, &quot;key&quot;: &quot;control&quot;}, {&quot;variables&quot;: [], &quot;id&quot;: &quot;20233267869&quot;, &quot;key&quot;: &quot;treatment&quot;}], &quot;id&quot;: &quot;20194668672&quot;, &quot;key&quot;: &quot;recommended_plan_in_signup&quot;, &quot;layerId&quot;: &quot;20231804245&quot;, &quot;trafficAllocation&quot;: [{&quot;entityId&quot;: &quot;20233267869&quot;, &quot;endOfRange&quot;: 2500}, {&quot;entityId&quot;: &quot;&quot;, &quot;endOfRange&quot;: 5000}, {&quot;entityId&quot;: &quot;20227754799&quot;, &quot;endOfRange&quot;: 7500}, {&quot;entityId&quot;: &quot;&quot;, &quot;endOfRange&quot;: 10000}], &quot;forcedVariations&quot;: {&quot;d0c8cbf56b61c99517936207d280de0c&quot;: &quot;treatment&quot;}}, {&quot;status&quot;: &quot;Running&quot;, &quot;audienceIds&quot;: [], &quot;variations&quot;: [{&quot;variables&quot;: [], &quot;id&quot;: &quot;20233300304&quot;, &quot;key&quot;: &quot;launch_code_variation&quot;}, {&quot;variables&quot;: [], &quot;id&quot;: &quot;20227370325&quot;, &quot;key&quot;: &quot;control&quot;}], &quot;id&quot;: &quot;20206000276&quot;, &quot;key&quot;: &quot;launch_code_verification&quot;, &quot;layerId&quot;: &quot;20233240262&quot;, &quot;trafficAllocation&quot;: [{&quot;entityId&quot;: &quot;20233300304&quot;, &quot;endOfRange&quot;: 2500}, {&quot;entityId&quot;: &quot;20227370325&quot;, &quot;endOfRange&quot;: 5000}, {&quot;entityId&quot;: &quot;&quot;, &quot;endOfRange&quot;: 10000}], &quot;forcedVariations&quot;: {}}], &quot;audiences&quot;: [{&quot;conditions&quot;: &quot;[\\&quot;or\\&quot;, {\\&quot;match\\&quot;: \\&quot;exact\\&quot;, \\&quot;name\\&quot;: \\&quot;$opt_dummy_attribute\\&quot;, \\&quot;type\\&quot;: \\&quot;custom_attribute\\&quot;, \\&quot;value\\&quot;: \\&quot;$opt_dummy_value\\&quot;}]&quot;, &quot;id&quot;: &quot;$opt_dummy_audience&quot;, &quot;name&quot;: &quot;Optimizely-Generated Audience for Backwards Compatibility&quot;}], &quot;groups&quot;: [], &quot;attributes&quot;: [{&quot;id&quot;: &quot;16822470375&quot;, &quot;key&quot;: &quot;user_id&quot;}, {&quot;id&quot;: &quot;17143601254&quot;, &quot;key&quot;: &quot;spammy&quot;}, {&quot;id&quot;: &quot;18175660309&quot;, &quot;key&quot;: &quot;organization_plan&quot;}, {&quot;id&quot;: &quot;18813001570&quot;, &quot;key&quot;: &quot;is_logged_in&quot;}, {&quot;id&quot;: &quot;19073851829&quot;, &quot;key&quot;: &quot;geo&quot;}, {&quot;id&quot;: &quot;20175462351&quot;, &quot;key&quot;: &quot;requestedCurrency&quot;}], &quot;botFiltering&quot;: false, &quot;accountId&quot;: &quot;16737760170&quot;, &quot;events&quot;: [{&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;17911811441&quot;, &quot;key&quot;: &quot;hydro_click.dashboard.teacher_toolbox_cta&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18124116703&quot;, &quot;key&quot;: &quot;submit.organizations.complete_sign_up&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18145892387&quot;, &quot;key&quot;: &quot;no_metric.tracked_outside_of_optimizely&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18178755568&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.add_repo&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18180553241&quot;, &quot;key&quot;: &quot;submit.repository_imports.create&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18186103728&quot;, &quot;key&quot;: &quot;click.help.learn_more_about_repository_creation&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18188530140&quot;, &quot;key&quot;: &quot;test_event.do_not_use_in_production&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18191963644&quot;, &quot;key&quot;: &quot;click.empty_org_repo_cta.transfer_repository&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18195612788&quot;, &quot;key&quot;: &quot;click.empty_org_repo_cta.import_repository&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18210945499&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.invite_members&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18211063248&quot;, &quot;key&quot;: &quot;click.empty_org_repo_cta.create_repository&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18215721889&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.update_profile&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18224360785&quot;, &quot;key&quot;: &quot;click.org_onboarding_checklist.dismiss&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18234832286&quot;, &quot;key&quot;: &quot;submit.organization_activation.complete&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18252392383&quot;, &quot;key&quot;: &quot;submit.org_repository.create&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18257551537&quot;, &quot;key&quot;: &quot;submit.org_member_invitation.create&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18259522260&quot;, &quot;key&quot;: &quot;submit.organization_profile.update&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18564603625&quot;, &quot;key&quot;: &quot;view.classroom_select_organization&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18568612016&quot;, &quot;key&quot;: &quot;click.classroom_sign_in_click&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18572592540&quot;, &quot;key&quot;: &quot;view.classroom_name&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18574203855&quot;, &quot;key&quot;: &quot;click.classroom_create_organization&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18582053415&quot;, &quot;key&quot;: &quot;click.classroom_select_organization&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18589463420&quot;, &quot;key&quot;: &quot;click.classroom_create_classroom&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18591323364&quot;, &quot;key&quot;: &quot;click.classroom_create_first_classroom&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18591652321&quot;, &quot;key&quot;: &quot;click.classroom_grant_access&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;18607131425&quot;, &quot;key&quot;: &quot;view.classroom_creation&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;18831680583&quot;, &quot;key&quot;: &quot;upgrade_account_plan&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19064064515&quot;, &quot;key&quot;: &quot;click.signup&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19075373687&quot;, &quot;key&quot;: &quot;click.view_account_billing_page&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19077355841&quot;, &quot;key&quot;: &quot;click.dismiss_signup_prompt&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19079713938&quot;, &quot;key&quot;: &quot;click.contact_sales&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19120963070&quot;, &quot;key&quot;: &quot;click.compare_account_plans&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19151690317&quot;, &quot;key&quot;: &quot;click.upgrade_account_cta&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19424193129&quot;, &quot;key&quot;: &quot;click.open_account_switcher&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19520330825&quot;, &quot;key&quot;: &quot;click.visit_account_profile&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19540970635&quot;, &quot;key&quot;: &quot;click.switch_account_context&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19730198868&quot;, &quot;key&quot;: &quot;submit.homepage_signup&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19820830627&quot;, &quot;key&quot;: &quot;click.homepage_signup&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;19988571001&quot;, &quot;key&quot;: &quot;click.create_enterprise_trial&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20036538294&quot;, &quot;key&quot;: &quot;click.create_organization_team&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20040653299&quot;, &quot;key&quot;: &quot;click.input_enterprise_trial_form&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20062030003&quot;, &quot;key&quot;: &quot;click.continue_with_team&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20068947153&quot;, &quot;key&quot;: &quot;click.create_organization_free&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20086636658&quot;, &quot;key&quot;: &quot;click.signup_continue.username&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20091648988&quot;, &quot;key&quot;: &quot;click.signup_continue.create_account&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20103637615&quot;, &quot;key&quot;: &quot;click.signup_continue.email&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20111574253&quot;, &quot;key&quot;: &quot;click.signup_continue.password&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20120044111&quot;, &quot;key&quot;: &quot;view.pricing_page&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20152062109&quot;, &quot;key&quot;: &quot;submit.create_account&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20165800992&quot;, &quot;key&quot;: &quot;submit.upgrade_payment_form&quot;}, {&quot;experimentIds&quot;: [], &quot;id&quot;: &quot;20171520319&quot;, &quot;key&quot;: &quot;submit.create_organization&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20222645674&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.discuss_your_needs&quot;}, {&quot;experimentIds&quot;: [&quot;20206000276&quot;], &quot;id&quot;: &quot;20227443657&quot;, &quot;key&quot;: &quot;submit.verify_primary_user_email&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20234607160&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.try_enterprise&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20238175784&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.team&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20239847212&quot;, &quot;key&quot;: &quot;click.recommended_plan_in_signup.continue_free&quot;}, {&quot;experimentIds&quot;: [&quot;20194668672&quot;], &quot;id&quot;: &quot;20251097193&quot;, &quot;key&quot;: &quot;recommended_plan&quot;}], &quot;revision&quot;: &quot;699&quot;}\" /> <!-- To prevent page flashing, the optimizely JS needs to be loaded in the <head> tag before the DOM renders --> <meta name=\"hostname\" content=\"github.com\"> <meta name=\"user-login\" content=\"\"> <meta name=\"expected-hostname\" content=\"github.com\"> <meta name=\"enabled-features\" content=\"MARKETPLACE_PENDING_INSTALLATIONS\"> <meta http-equiv=\"x-pjax-version\" content=\"87f8658c32da34e071cd8c5cc1982a2958ea21e57a2527dc58c4ff59fdb6c0ab\"> <meta name=\"go-import\" content=\"github.com/bitcoin/bitcoin git https://github.com/bitcoin/bitcoin.git\"> <meta name=\"octolytics-dimension-user_id\" content=\"528860\" /><meta name=\"octolytics-dimension-user_login\" content=\"bitcoin\" /><meta name=\"octolytics-dimension-repository_id\" content=\"1181927\" /><meta name=\"octolytics-dimension-repository_nwo\" content=\"bitcoin/bitcoin\" /><meta name=\"octolytics-dimension-repository_public\" content=\"true\" /><meta name=\"octolytics-dimension-repository_is_fork\" content=\"false\" /><meta name=\"octolytics-dimension-repository_network_root_id\" content=\"1181927\" /><meta name=\"octolytics-dimension-repository_network_root_nwo\" content=\"bitcoin/bitcoin\" /> <meta name=\"browser-stats-url\" content=\"https://api.github.com/_private/browser/stats\"> <meta name=\"browser-errors-url\" content=\"https://api.github.com/_private/browser/errors\"> <meta name=\"browser-optimizely-client-errors-url\" content=\"https://api.github.com/_private/browser/optimizely_client/errors\"> <meta name=\"theme-color\" content=\"#1e2327\"> <meta name=\"color-scheme\" content=\"light dark\" /> <meta name=\"enabled-homepage-translation-languages\" content=\"\"> </head> <body class=\"logged-out env-production page-responsive\" style=\"word-wrap: break-word;\"> <div class=\"position-relative js-header-wrapper \"> <a href=\"#start-of-content\" class=\"px-2 py-4 color-bg-info-inverse color-text-white show-on-focus js-skip-to-content\">Skip to content</a> <span data-view-component=\"true\" class=\"progress-pjax-loader width-full js-pjax-loader-bar Progress position-fixed\"> <span style=\"background-color: #79b8ff;width: 0%;\" data-view-component=\"true\" class=\"Progress-item progress-pjax-loader-bar\"></span> </span> <header class=\"Header-old header-logged-out js-details-container Details position-relative f4 py-2\" role=\"banner\"> <div class=\"container-xl d-lg-flex flex-items-center p-responsive\"> <div class=\"d-flex flex-justify-between flex-items-center\"> <a class=\"mr-4\" href=\"https://github.com/\" aria-label=\"Homepage\" data-ga-click=\"(Logged out) Header, go to homepage, icon:logo-wordmark\"> <svg height=\"32\" class=\"octicon octicon-mark-github color-text-white\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"32\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z\"></path></svg> </a> <div class=\"d-lg-none css-truncate css-truncate-target width-fit p-2\"> </div> <div class=\"d-flex flex-items-center\"> <a href=\"/signup?ref_cta=Sign+up&amp;ref_loc=header+logged+out&amp;ref_page=%2F%3Cuser-name%3E%2F%3Crepo-name%3E&amp;source=header-repo\" class=\"d-inline-block d-lg-none f5 color-text-white no-underline border color-border-tertiary rounded-2 px-2 py-1 mr-3 mr-sm-5\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;site header&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;SIGN_UP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"eede25cdbef5dc840192e10fcde2597001ef5002d277864f39a27424031daa40\" > Sign&nbsp;up </a> <button class=\"btn-link d-lg-none mt-1 js-details-target\" type=\"button\" aria-label=\"Toggle navigation\" aria-expanded=\"false\"> <svg height=\"24\" class=\"octicon octicon-three-bars color-text-white\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"24\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M1 2.75A.75.75 0 011.75 2h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 2.75zm0 5A.75.75 0 011.75 7h12.5a.75.75 0 110 1.5H1.75A.75.75 0 011 7.75zM1.75 12a.75.75 0 100 1.5h12.5a.75.75 0 100-1.5H1.75z\"></path></svg> </button> </div> </div> <div class=\"HeaderMenu HeaderMenu--logged-out position-fixed top-0 right-0 bottom-0 height-fit position-lg-relative d-lg-flex flex-justify-between flex-items-center flex-auto\"> <div class=\"d-flex d-lg-none flex-justify-end border-bottom color-bg-secondary p-3\"> <button class=\"btn-link js-details-target\" type=\"button\" aria-label=\"Toggle navigation\" aria-expanded=\"false\"> <svg height=\"24\" class=\"octicon octicon-x color-text-secondary\" viewBox=\"0 0 24 24\" version=\"1.1\" width=\"24\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M5.72 5.72a.75.75 0 011.06 0L12 10.94l5.22-5.22a.75.75 0 111.06 1.06L13.06 12l5.22 5.22a.75.75 0 11-1.06 1.06L12 13.06l-5.22 5.22a.75.75 0 01-1.06-1.06L10.94 12 5.72 6.78a.75.75 0 010-1.06z\"></path></svg> </button> </div> <nav class=\"mt-0 px-3 px-lg-0 mb-5 mb-lg-0\" aria-label=\"Global\"> <ul class=\"d-lg-flex list-style-none\"> <li class=\"d-block d-lg-flex flex-lg-nowrap flex-lg-items-center border-bottom border-lg-bottom-0 mr-0 mr-lg-3 edge-item-fix position-relative flex-wrap flex-justify-between d-flex flex-items-center \"> <details class=\"HeaderMenu-details details-overlay details-reset width-full\"> <summary class=\"HeaderMenu-summary HeaderMenu-link px-0 py-3 border-0 no-wrap d-block d-lg-inline-block\"> Why GitHub? <svg x=\"0px\" y=\"0px\" viewBox=\"0 0 14 8\" xml:space=\"preserve\" fill=\"none\" class=\"icon-chevon-down-mktg position-absolute position-lg-relative\"> <path d=\"M1,1l6.2,6L13,1\"></path> </svg> </summary> <div class=\"dropdown-menu flex-auto rounded px-0 mt-0 pb-4 p-lg-4 position-relative position-lg-absolute left-0 left-lg-n4\"> <a href=\"/features\" class=\"py-2 lh-condensed-ultra d-block Link--primary no-underline h5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Features\">Features <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a> <ul class=\"list-style-none f5 pb-3\"> <li class=\"edge-item-fix\"><a href=\"/mobile\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Mobile <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/actions\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Actions <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/codespaces\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Codespaces <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/packages\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Packages <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/security\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Security <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/code-review/\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Code review <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/project-management/\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Project management <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/features/integrations\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">Integrations <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> <ul class=\"list-style-none mb-0 border-lg-top pt-lg-3\"> <li class=\"edge-item-fix\"><a href=\"/sponsors\" class=\"py-2 lh-condensed-ultra d-block no-underline Link--primary no-underline h5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Sponsors\">GitHub Sponsors <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/customer-stories\" class=\"py-2 lh-condensed-ultra d-block no-underline Link--primary no-underline h5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Customer stories\">Customer stories<span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> </div> </details> </li> <li class=\"border-bottom border-lg-bottom-0 mr-0 mr-lg-3\"> <a href=\"/team\" class=\"HeaderMenu-link no-underline py-3 d-block d-lg-inline-block\" data-ga-click=\"(Logged out) Header, go to Team\">Team</a> </li> <li class=\"border-bottom border-lg-bottom-0 mr-0 mr-lg-3\"> <a href=\"/enterprise\" class=\"HeaderMenu-link no-underline py-3 d-block d-lg-inline-block\" data-ga-click=\"(Logged out) Header, go to Enterprise\">Enterprise</a> </li> <li class=\"d-block d-lg-flex flex-lg-nowrap flex-lg-items-center border-bottom border-lg-bottom-0 mr-0 mr-lg-3 edge-item-fix position-relative flex-wrap flex-justify-between d-flex flex-items-center \"> <details class=\"HeaderMenu-details details-overlay details-reset width-full\"> <summary class=\"HeaderMenu-summary HeaderMenu-link px-0 py-3 border-0 no-wrap d-block d-lg-inline-block\"> Explore <svg x=\"0px\" y=\"0px\" viewBox=\"0 0 14 8\" xml:space=\"preserve\" fill=\"none\" class=\"icon-chevon-down-mktg position-absolute position-lg-relative\"> <path d=\"M1,1l6.2,6L13,1\"></path> </svg> </summary> <div class=\"dropdown-menu flex-auto rounded px-0 pt-2 pb-0 mt-0 pb-4 p-lg-4 position-relative position-lg-absolute left-0 left-lg-n4\"> <ul class=\"list-style-none mb-3\"> <li class=\"edge-item-fix\"><a href=\"/explore\" class=\"py-2 lh-condensed-ultra d-block Link--primary no-underline h5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Explore\">Explore GitHub <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> <h4 class=\"color-text-tertiary text-normal text-mono f5 mb-2 border-lg-top pt-lg-3\">Learn and contribute</h4> <ul class=\"list-style-none mb-3\"> <li class=\"edge-item-fix\"><a href=\"/topics\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Topics\">Topics <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/collections\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Collections\">Collections <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"/trending\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Trending\">Trending <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://lab.github.com/\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Learning lab\">Learning Lab <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://opensource.guide\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Open source guides\">Open source guides <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> <h4 class=\"color-text-tertiary text-normal text-mono f5 mb-2 border-lg-top pt-lg-3\">Connect with others</h4> <ul class=\"list-style-none mb-0\"> <li class=\"edge-item-fix\"><a href=\"https://github.com/readme\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\">The ReadME Project <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://github.com/events\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Events\">Events <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://github.community\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Community forum\">Community forum <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://education.github.com\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to GitHub Education\">GitHub Education <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://stars.github.com\" class=\"py-2 pb-0 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to GitHub Stars Program\">GitHub Stars program <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> </div> </details> </li> <li class=\"border-bottom border-lg-bottom-0 mr-0 mr-lg-3\"> <a href=\"/marketplace\" class=\"HeaderMenu-link no-underline py-3 d-block d-lg-inline-block\" data-ga-click=\"(Logged out) Header, go to Marketplace\">Marketplace</a> </li> <li class=\"d-block d-lg-flex flex-lg-nowrap flex-lg-items-center border-bottom border-lg-bottom-0 mr-0 mr-lg-3 edge-item-fix position-relative flex-wrap flex-justify-between d-flex flex-items-center \"> <details class=\"HeaderMenu-details details-overlay details-reset width-full\"> <summary class=\"HeaderMenu-summary HeaderMenu-link px-0 py-3 border-0 no-wrap d-block d-lg-inline-block\"> Pricing <svg x=\"0px\" y=\"0px\" viewBox=\"0 0 14 8\" xml:space=\"preserve\" fill=\"none\" class=\"icon-chevon-down-mktg position-absolute position-lg-relative\"> <path d=\"M1,1l6.2,6L13,1\"></path> </svg> </summary> <div class=\"dropdown-menu flex-auto rounded px-0 pt-2 pb-4 mt-0 p-lg-4 position-relative position-lg-absolute left-0 left-lg-n4\"> <a href=\"/pricing\" class=\"pb-2 lh-condensed-ultra d-block Link--primary no-underline h5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Pricing\">Plans <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a> <ul class=\"list-style-none mb-3\"> <li class=\"edge-item-fix\"><a href=\"/pricing#feature-comparison\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Compare plans\">Compare plans <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> <li class=\"edge-item-fix\"><a href=\"https://enterprise.github.com/contact\" class=\"py-2 lh-condensed-ultra d-block Link--secondary no-underline f5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Contact Sales\">Contact Sales <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> <ul class=\"list-style-none mb-0 border-lg-top pt-lg-3\"> <li class=\"edge-item-fix\"><a href=\"https://education.github.com\" class=\"py-2 pb-0 lh-condensed-ultra d-block no-underline Link--primary no-underline h5 Bump-link--hover\" data-ga-click=\"(Logged out) Header, go to Education\">Education <span class=\"Bump-link-symbol float-right text-normal color-text-tertiary pr-3\">&rarr;</span></a></li> </ul> </div> </details> </li> </ul> </nav> <div class=\"d-lg-flex flex-items-center px-3 px-lg-0 text-center text-lg-left\"> <div class=\"d-lg-flex min-width-0 mb-3 mb-lg-0\"> <div class=\"header-search flex-auto js-site-search position-relative flex-self-stretch flex-md-self-auto mb-3 mb-md-0 mr-0 mr-md-3 scoped-search site-scoped-search js-jump-to\" > <div class=\"position-relative\"> </option></form><form class=\"js-site-search-form\" role=\"search\" aria-label=\"Site\" data-scope-type=\"Repository\" data-scope-id=\"1181927\" data-scoped-search-url=\"/bitcoin/bitcoin/search\" data-owner-scoped-search-url=\"/orgs/bitcoin/search\" data-unscoped-search-url=\"/search\" action=\"/bitcoin/bitcoin/search\" accept-charset=\"UTF-8\" method=\"get\"> <label class=\"form-control input-sm header-search-wrapper p-0 js-chromeless-input-container header-search-wrapper-jump-to position-relative d-flex flex-justify-between flex-items-center\"> <input type=\"text\" class=\"form-control input-sm header-search-input jump-to-field js-jump-to-field js-site-search-focus js-site-search-field is-clearable\" data-hotkey=s,/ name=\"q\" value=\"\" placeholder=\"Search\" data-unscoped-placeholder=\"Search GitHub\" data-scoped-placeholder=\"Search\" autocapitalize=\"off\" role=\"combobox\" aria-haspopup=\"listbox\" aria-expanded=\"false\" aria-autocomplete=\"list\" aria-controls=\"jump-to-results\" aria-label=\"Search\" data-jump-to-suggestions-path=\"/_graphql/GetSuggestedNavigationDestinations\" spellcheck=\"false\" autocomplete=\"off\" > <input type=\"hidden\" data-csrf=\"true\" class=\"js-data-jump-to-suggestions-path-csrf\" value=\"5Vz+KwHFpY+J/1FyVmfqIZ4M6wdnxpstJ/a9A4pUvywFejCqEhiFQstUqQqLYjfCxZeByXYN94sZS82RNP8SOA==\" /> <input type=\"hidden\" class=\"js-site-search-type-field\" name=\"type\" > <img src=\"https://github.githubassets.com/images/search-key-slash.svg\" alt=\"\" class=\"mr-2 header-search-key-slash\"> <div class=\"Box position-absolute overflow-hidden d-none jump-to-suggestions js-jump-to-suggestions-container\"> <ul class=\"d-none js-jump-to-suggestions-template-container\"> <li class=\"d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-suggestion\" role=\"option\"> <a tabindex=\"-1\" class=\"no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2\" href=\"\" data-item-type=\"suggestion\"> <div class=\"jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none\"> <svg height=\"16\" width=\"16\" class=\"octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none\" title=\"Repository\" aria-label=\"Repository\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none\" title=\"Project\" aria-label=\"Project\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none\" title=\"Search\" aria-label=\"Search\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z\"></path></svg> </div> <img class=\"avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none\" alt=\"\" aria-label=\"Team\" src=\"\" width=\"28\" height=\"28\"> <div class=\"jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target\"> </div> <div class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search\"> <span class=\"js-jump-to-badge-search-text-default d-none\" aria-label=\"in this repository\"> In this repository </span> <span class=\"js-jump-to-badge-search-text-global d-none\" aria-label=\"in all of GitHub\"> All GitHub </span> <span aria-hidden=\"true\" class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> <div aria-hidden=\"true\" class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump\"> Jump to <span class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> </a> </li> </ul> <ul class=\"d-none js-jump-to-no-results-template-container\"> <li class=\"d-flex flex-justify-center flex-items-center f5 d-none js-jump-to-suggestion p-2\"> <span class=\"color-text-secondary\">No suggested jump to results</span> </li> </ul> <ul id=\"jump-to-results\" role=\"listbox\" class=\"p-0 m-0 js-navigation-container jump-to-suggestions-results-container js-jump-to-suggestions-results-container\"> <li class=\"d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-scoped-search d-none\" role=\"option\"> <a tabindex=\"-1\" class=\"no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2\" href=\"\" data-item-type=\"scoped_search\"> <div class=\"jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none\"> <svg height=\"16\" width=\"16\" class=\"octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none\" title=\"Repository\" aria-label=\"Repository\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none\" title=\"Project\" aria-label=\"Project\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none\" title=\"Search\" aria-label=\"Search\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z\"></path></svg> </div> <img class=\"avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none\" alt=\"\" aria-label=\"Team\" src=\"\" width=\"28\" height=\"28\"> <div class=\"jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target\"> </div> <div class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search\"> <span class=\"js-jump-to-badge-search-text-default d-none\" aria-label=\"in this repository\"> In this repository </span> <span class=\"js-jump-to-badge-search-text-global d-none\" aria-label=\"in all of GitHub\"> All GitHub </span> <span aria-hidden=\"true\" class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> <div aria-hidden=\"true\" class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump\"> Jump to <span class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> </a> </li> <li class=\"d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-owner-scoped-search d-none\" role=\"option\"> <a tabindex=\"-1\" class=\"no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2\" href=\"\" data-item-type=\"owner_scoped_search\"> <div class=\"jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none\"> <svg height=\"16\" width=\"16\" class=\"octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none\" title=\"Repository\" aria-label=\"Repository\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none\" title=\"Project\" aria-label=\"Project\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none\" title=\"Search\" aria-label=\"Search\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z\"></path></svg> </div> <img class=\"avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none\" alt=\"\" aria-label=\"Team\" src=\"\" width=\"28\" height=\"28\"> <div class=\"jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target\"> </div> <div class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search\"> <span class=\"js-jump-to-badge-search-text-default d-none\" aria-label=\"in this organization\"> In this organization </span> <span class=\"js-jump-to-badge-search-text-global d-none\" aria-label=\"in all of GitHub\"> All GitHub </span> <span aria-hidden=\"true\" class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> <div aria-hidden=\"true\" class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump\"> Jump to <span class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> </a> </li> <li class=\"d-flex flex-justify-start flex-items-center p-0 f5 navigation-item js-navigation-item js-jump-to-global-search d-none\" role=\"option\"> <a tabindex=\"-1\" class=\"no-underline d-flex flex-auto flex-items-center jump-to-suggestions-path js-jump-to-suggestion-path js-navigation-open p-2\" href=\"\" data-item-type=\"global_search\"> <div class=\"jump-to-octicon js-jump-to-octicon flex-shrink-0 mr-2 text-center d-none\"> <svg height=\"16\" width=\"16\" class=\"octicon octicon-repo flex-shrink-0 js-jump-to-octicon-repo d-none\" title=\"Repository\" aria-label=\"Repository\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-project flex-shrink-0 js-jump-to-octicon-project d-none\" title=\"Project\" aria-label=\"Project\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z\"></path></svg> <svg height=\"16\" width=\"16\" class=\"octicon octicon-search flex-shrink-0 js-jump-to-octicon-search d-none\" title=\"Search\" aria-label=\"Search\" viewBox=\"0 0 16 16\" version=\"1.1\" role=\"img\"><path fill-rule=\"evenodd\" d=\"M11.5 7a4.499 4.499 0 11-8.998 0A4.499 4.499 0 0111.5 7zm-.82 4.74a6 6 0 111.06-1.06l3.04 3.04a.75.75 0 11-1.06 1.06l-3.04-3.04z\"></path></svg> </div> <img class=\"avatar mr-2 flex-shrink-0 js-jump-to-suggestion-avatar d-none\" alt=\"\" aria-label=\"Team\" src=\"\" width=\"28\" height=\"28\"> <div class=\"jump-to-suggestion-name js-jump-to-suggestion-name flex-auto overflow-hidden text-left no-wrap css-truncate css-truncate-target\"> </div> <div class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none js-jump-to-badge-search\"> <span class=\"js-jump-to-badge-search-text-default d-none\" aria-label=\"in this repository\"> In this repository </span> <span class=\"js-jump-to-badge-search-text-global d-none\" aria-label=\"in all of GitHub\"> All GitHub </span> <span aria-hidden=\"true\" class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> <div aria-hidden=\"true\" class=\"border rounded-1 flex-shrink-0 color-bg-tertiary px-1 color-text-tertiary ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump\"> Jump to <span class=\"d-inline-block ml-1 v-align-middle\">↵</span> </div> </a> </li> </ul> </div> </label> </form> </div> </div> </div> <div class=\"position-relative mr-3\"> <a href=\"/login?return_to=%2Fbitcoin%2Fbitcoin\" class=\"HeaderMenu-link flex-shrink-0 no-underline\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;site header menu&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;SIGN_UP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"644092c20f517603530d2054f8f0fa1898daddbfb04c9da7fcd7b8ad8ba103ca\" data-ga-click=\"(Logged out) Header, clicked Sign in, text:sign-in\"> Sign in </a> </div> <a href=\"/signup?ref_cta=Sign+up&amp;ref_loc=header+logged+out&amp;ref_page=%2F%3Cuser-name%3E%2F%3Crepo-name%3E&amp;source=header-repo&amp;source_repo=bitcoin%2Fbitcoin\" class=\"HeaderMenu-link flex-shrink-0 d-inline-block no-underline border color-border-tertiary rounded px-2 py-1\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;site header menu&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;SIGN_UP&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"644092c20f517603530d2054f8f0fa1898daddbfb04c9da7fcd7b8ad8ba103ca\" data-hydro-click=\"{&quot;event_type&quot;:&quot;analytics.event&quot;,&quot;payload&quot;:{&quot;category&quot;:&quot;Sign up&quot;,&quot;action&quot;:&quot;click to sign up for account&quot;,&quot;label&quot;:&quot;ref_page:/&lt;user-name&gt;/&lt;repo-name&gt;;ref_cta:Sign up;ref_loc:header logged out&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"c1787dae27c29f3ef7c3a1fdae1614d04f34ed70f05c5e7597273c3e7048efb1\" > Sign up </a> </div> </div> </div> </header> </div> <div id=\"start-of-content\" class=\"show-on-focus\"></div> <div data-pjax-replace id=\"js-flash-container\"> <template class=\"js-flash-template\"> <div class=\"flash flash-full {{ className }}\"> <div class=\" px-2\" > <button class=\"flash-close js-flash-close\" type=\"button\" aria-label=\"Dismiss this message\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-x\"> <path fill-rule=\"evenodd\" d=\"M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z\"></path> </svg> </button> <div>{{ message }}</div> </div> </div> </template> </div> <include-fragment class=\"js-notification-shelf-include-fragment\" data-base-src=\"https://github.com/notifications/beta/shelf\"></include-fragment> <div class=\"application-main \" data-commit-hovercards-enabled data-discussion-hovercards-enabled data-issue-and-pr-hovercards-enabled > <div itemscope itemtype=\"http://schema.org/SoftwareSourceCode\" class=\"\"> <main id=\"js-repo-pjax-container\" data-pjax-container > <div class=\"hx_page-header-bg pt-3 hide-full-screen mb-5\"> <div class=\"d-flex mb-3 px-3 px-md-4 px-lg-5\"> <div class=\"flex-auto min-width-0 width-fit mr-3\"> <h1 class=\" d-flex flex-wrap flex-items-center break-word f3 text-normal\"> <svg class=\"octicon octicon-repo color-text-secondary mr-2\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M2 2.5A2.5 2.5 0 014.5 0h8.75a.75.75 0 01.75.75v12.5a.75.75 0 01-.75.75h-2.5a.75.75 0 110-1.5h1.75v-2h-8a1 1 0 00-.714 1.7.75.75 0 01-1.072 1.05A2.495 2.495 0 012 11.5v-9zm10.5-1V9h-8c-.356 0-.694.074-1 .208V2.5a1 1 0 011-1h8zM5 12.25v3.25a.25.25 0 00.4.2l1.45-1.087a.25.25 0 01.3 0L8.6 15.7a.25.25 0 00.4-.2v-3.25a.25.25 0 00-.25-.25h-3.5a.25.25 0 00-.25.25z\"></path></svg> <span class=\"author flex-self-stretch\" itemprop=\"author\"> <a class=\"url fn\" rel=\"author\" data-hovercard-type=\"organization\" data-hovercard-url=\"/orgs/bitcoin/hovercard\" href=\"/bitcoin\">bitcoin</a> </span> <span class=\"mx-1 flex-self-stretch color-text-secondary\">/</span> <strong itemprop=\"name\" class=\"mr-2 flex-self-stretch\"> <a data-pjax=\"#js-repo-pjax-container\" href=\"/bitcoin/bitcoin\">bitcoin</a> </strong> </h1> </div> <ul class=\"pagehead-actions flex-shrink-0 d-none d-md-inline\" style=\"padding: 2px 0;\"> <li> <a class=\"tooltipped tooltipped-s btn btn-sm\" aria-label=\"You must be signed in to change notification settings\" rel=\"nofollow\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;notification subscription menu watch&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"e10ea9b99c78014bc22224bdb805ecdc9d06f57b1acce6a5430a56f1cfa6aa7d\" href=\"/login?return_to=%2Fbitcoin%2Fbitcoin\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-bell\"> <path d=\"M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z\"></path><path fill-rule=\"evenodd\" d=\"M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z\"></path> </svg> Notifications </a> </li> <li> <a class=\"btn btn-sm btn-with-count tooltipped tooltipped-s\" aria-label=\"You must be signed in to star a repository\" rel=\"nofollow\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;star button&quot;,&quot;repository_id&quot;:1181927,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"063182d321a46eaef39541cfa8541885c6ec9053971b8659c00a4e5a865e8603\" href=\"/login?return_to=%2Fbitcoin%2Fbitcoin\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-star v-align-text-bottom mr-1\"> <path fill-rule=\"evenodd\" d=\"M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z\"></path> </svg> <span data-view-component=\"true\"> Star </span></a> <a class=\"social-count js-social-count\" href=\"/bitcoin/bitcoin/stargazers\" aria-label=\"55213 users starred this repository\"> 55.2k </a> </li> <li> <a class=\"btn btn-sm btn-with-count tooltipped tooltipped-s\" aria-label=\"You must be signed in to fork a repository\" rel=\"nofollow\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;repo details fork button&quot;,&quot;repository_id&quot;:1181927,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"8b549d22536b537c0103309c960956b6f5289ce501f9b885723bd746a0e785f1\" href=\"/login?return_to=%2Fbitcoin%2Fbitcoin\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-repo-forked\"> <path fill-rule=\"evenodd\" d=\"M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z\"></path> </svg> Fork </a> <a href=\"/bitcoin/bitcoin/network/members\" class=\"social-count\" aria-label=\"29215 users forked this repository\"> 29.2k </a> </li> </ul> </div> <div class=\"d-block d-md-none mb-2 px-3 px-md-4 px-lg-5\"> <p class=\"f4 mb-3\"> Bitcoin Core integration/staging tree </p> <div class=\"mb-2 d-flex flex-items-center\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-link flex-shrink-0 mr-2\"> <path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path> </svg> <span class=\"flex-auto min-width-0 css-truncate css-truncate-target width-fit\"> <a title=\"https://bitcoincore.org/en/download\" role=\"link\" target=\"_blank\" class=\"text-bold\" rel=\"noopener noreferrer\" href=\"https://bitcoincore.org/en/download\">bitcoincore.org/en/download</a> </span> </div> <div class=\"mb-2\"> <a href=\"/bitcoin/bitcoin/blob/master/COPYING\" class=\"Link--muted\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-law mr-1\"> <path fill-rule=\"evenodd\" d=\"M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z\"></path> </svg> MIT License </a> </div> <div class=\"mb-3\"> <a class=\"Link--secondary no-underline mr-3\" href=\"/bitcoin/bitcoin/stargazers\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-star mr-1\"> <path fill-rule=\"evenodd\" d=\"M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z\"></path> </svg> <span class=\"text-bold\">55.2k</span> stars </a> <a class=\"Link--secondary no-underline\" href=\"/bitcoin/bitcoin/network/members\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-repo-forked mr-1\"> <path fill-rule=\"evenodd\" d=\"M5 3.25a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm0 2.122a2.25 2.25 0 10-1.5 0v.878A2.25 2.25 0 005.75 8.5h1.5v2.128a2.251 2.251 0 101.5 0V8.5h1.5a2.25 2.25 0 002.25-2.25v-.878a2.25 2.25 0 10-1.5 0v.878a.75.75 0 01-.75.75h-4.5A.75.75 0 015 6.25v-.878zm3.75 7.378a.75.75 0 11-1.5 0 .75.75 0 011.5 0zm3-8.75a.75.75 0 100-1.5.75.75 0 000 1.5z\"></path> </svg> <span class=\"text-bold\">29.2k</span> forks </a> </div> <div class=\"d-flex\"> <div class=\"flex-1 mr-2\"> <a class=\"btn btn-sm btn-block tooltipped tooltipped-s\" aria-label=\"You must be signed in to star a repository\" rel=\"nofollow\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;star button&quot;,&quot;repository_id&quot;:1181927,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"063182d321a46eaef39541cfa8541885c6ec9053971b8659c00a4e5a865e8603\" href=\"/login?return_to=%2Fbitcoin%2Fbitcoin\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-star v-align-text-bottom mr-1\"> <path fill-rule=\"evenodd\" d=\"M8 .25a.75.75 0 01.673.418l1.882 3.815 4.21.612a.75.75 0 01.416 1.279l-3.046 2.97.719 4.192a.75.75 0 01-1.088.791L8 12.347l-3.766 1.98a.75.75 0 01-1.088-.79l.72-4.194L.818 6.374a.75.75 0 01.416-1.28l4.21-.611L7.327.668A.75.75 0 018 .25zm0 2.445L6.615 5.5a.75.75 0 01-.564.41l-3.097.45 2.24 2.184a.75.75 0 01.216.664l-.528 3.084 2.769-1.456a.75.75 0 01.698 0l2.77 1.456-.53-3.084a.75.75 0 01.216-.664l2.24-2.183-3.096-.45a.75.75 0 01-.564-.41L8 2.694v.001z\"></path> </svg> <span data-view-component=\"true\"> Star </span></a> </div> <div class=\"flex-1\"> <a class=\"tooltipped tooltipped-s btn btn-sm btn-block\" aria-label=\"You must be signed in to change notification settings\" rel=\"nofollow\" data-hydro-click=\"{&quot;event_type&quot;:&quot;authentication.click&quot;,&quot;payload&quot;:{&quot;location_in_page&quot;:&quot;notification subscription menu watch&quot;,&quot;repository_id&quot;:null,&quot;auth_type&quot;:&quot;LOG_IN&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"e10ea9b99c78014bc22224bdb805ecdc9d06f57b1acce6a5430a56f1cfa6aa7d\" href=\"/login?return_to=%2Fbitcoin%2Fbitcoin\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-bell\"> <path d=\"M8 16a2 2 0 001.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 008 16z\"></path><path fill-rule=\"evenodd\" d=\"M8 1.5A3.5 3.5 0 004.5 5v2.947c0 .346-.102.683-.294.97l-1.703 2.556a.018.018 0 00-.003.01l.001.006c0 .002.002.004.004.006a.017.017 0 00.006.004l.007.001h10.964l.007-.001a.016.016 0 00.006-.004.016.016 0 00.004-.006l.001-.007a.017.017 0 00-.003-.01l-1.703-2.554a1.75 1.75 0 01-.294-.97V5A3.5 3.5 0 008 1.5zM3 5a5 5 0 0110 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.518 1.518 0 0113.482 13H2.518a1.518 1.518 0 01-1.263-2.36l1.703-2.554A.25.25 0 003 7.947V5z\"></path> </svg> Notifications </a> </div> </div> </div> <nav data-pjax=\"#js-repo-pjax-container\" aria-label=\"Repository\" data-view-component=\"true\" class=\"js-repo-nav js-sidenav-container-pjax js-responsive-underlinenav overflow-hidden UnderlineNav px-3 px-md-4 px-lg-5\"> <ul data-view-component=\"true\" class=\"UnderlineNav-body list-style-none\"> <li data-view-component=\"true\" class=\"d-flex\"> <a href=\"/bitcoin/bitcoin\" data-tab-item=\"i0code-tab\" data-selected-links=\"repo_source repo_downloads repo_commits repo_releases repo_tags repo_branches repo_packages repo_deployments /bitcoin/bitcoin\" data-hotkey=\"g c\" data-ga-click=\"Repository, Navigation click, Code tab\" aria-current=\"page\" data-view-component=\"true\" class=\"UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item selected\"> <svg class=\"octicon octicon-code UnderlineNav-octicon d-none d-sm-inline\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M4.72 3.22a.75.75 0 011.06 1.06L2.06 8l3.72 3.72a.75.75 0 11-1.06 1.06L.47 8.53a.75.75 0 010-1.06l4.25-4.25zm6.56 0a.75.75 0 10-1.06 1.06L13.94 8l-3.72 3.72a.75.75 0 101.06 1.06l4.25-4.25a.75.75 0 000-1.06l-4.25-4.25z\"></path></svg> <span data-content=\"Code\">Code</span> <span title=\"Not available\" data-view-component=\"true\" class=\"Counter\"></span> </a></li> <li data-view-component=\"true\" class=\"d-flex\"> <a href=\"/bitcoin/bitcoin/issues\" data-tab-item=\"i1issues-tab\" data-selected-links=\"repo_issues repo_labels repo_milestones /bitcoin/bitcoin/issues\" data-hotkey=\"g i\" data-ga-click=\"Repository, Navigation click, Issues tab\" data-view-component=\"true\" class=\"UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item\"> <svg class=\"octicon octicon-issue-opened UnderlineNav-octicon d-none d-sm-inline\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M8 9.5a1.5 1.5 0 100-3 1.5 1.5 0 000 3z\"></path><path fill-rule=\"evenodd\" d=\"M8 0a8 8 0 100 16A8 8 0 008 0zM1.5 8a6.5 6.5 0 1113 0 6.5 6.5 0 01-13 0z\"></path></svg> <span data-content=\"Issues\">Issues</span> <span title=\"588\" data-view-component=\"true\" class=\"Counter\">588</span> </a></li> <li data-view-component=\"true\" class=\"d-flex\"> <a href=\"/bitcoin/bitcoin/pulls\" data-tab-item=\"i2pull-requests-tab\" data-selected-links=\"repo_pulls checks /bitcoin/bitcoin/pulls\" data-hotkey=\"g p\" data-ga-click=\"Repository, Navigation click, Pull requests tab\" data-view-component=\"true\" class=\"UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item\"> <svg class=\"octicon octicon-git-pull-request UnderlineNav-octicon d-none d-sm-inline\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.177 3.073L9.573.677A.25.25 0 0110 .854v4.792a.25.25 0 01-.427.177L7.177 3.427a.25.25 0 010-.354zM3.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122v5.256a2.251 2.251 0 11-1.5 0V5.372A2.25 2.25 0 011.5 3.25zM11 2.5h-1V4h1a1 1 0 011 1v5.628a2.251 2.251 0 101.5 0V5A2.5 2.5 0 0011 2.5zm1 10.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0zM3.75 12a.75.75 0 100 1.5.75.75 0 000-1.5z\"></path></svg> <span data-content=\"Pull requests\">Pull requests</span> <span title=\"402\" data-view-component=\"true\" class=\"Counter\">402</span> </a></li> <li data-view-component=\"true\" class=\"d-flex\"> <a href=\"/bitcoin/bitcoin/projects\" data-tab-item=\"i3projects-tab\" data-selected-links=\"repo_projects new_repo_project repo_project /bitcoin/bitcoin/projects\" data-hotkey=\"g b\" data-ga-click=\"Repository, Navigation click, Projects tab\" data-view-component=\"true\" class=\"UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item\"> <svg class=\"octicon octicon-project UnderlineNav-octicon d-none d-sm-inline\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M1.75 0A1.75 1.75 0 000 1.75v12.5C0 15.216.784 16 1.75 16h12.5A1.75 1.75 0 0016 14.25V1.75A1.75 1.75 0 0014.25 0H1.75zM1.5 1.75a.25.25 0 01.25-.25h12.5a.25.25 0 01.25.25v12.5a.25.25 0 01-.25.25H1.75a.25.25 0 01-.25-.25V1.75zM11.75 3a.75.75 0 00-.75.75v7.5a.75.75 0 001.5 0v-7.5a.75.75 0 00-.75-.75zm-8.25.75a.75.75 0 011.5 0v5.5a.75.75 0 01-1.5 0v-5.5zM8 3a.75.75 0 00-.75.75v3.5a.75.75 0 001.5 0v-3.5A.75.75 0 008 3z\"></path></svg> <span data-content=\"Projects\">Projects</span> <span title=\"7\" data-view-component=\"true\" class=\"Counter\">7</span> </a></li> <li data-view-component=\"true\" class=\"d-flex\"> <a href=\"/bitcoin/bitcoin/security\" data-tab-item=\"i4security-tab\" data-selected-links=\"security overview alerts policy token_scanning code_scanning /bitcoin/bitcoin/security\" data-hotkey=\"g s\" data-ga-click=\"Repository, Navigation click, Security tab\" data-view-component=\"true\" class=\"UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item\"> <svg class=\"octicon octicon-shield UnderlineNav-octicon d-none d-sm-inline\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.467.133a1.75 1.75 0 011.066 0l5.25 1.68A1.75 1.75 0 0115 3.48V7c0 1.566-.32 3.182-1.303 4.682-.983 1.498-2.585 2.813-5.032 3.855a1.7 1.7 0 01-1.33 0c-2.447-1.042-4.049-2.357-5.032-3.855C1.32 10.182 1 8.566 1 7V3.48a1.75 1.75 0 011.217-1.667l5.25-1.68zm.61 1.429a.25.25 0 00-.153 0l-5.25 1.68a.25.25 0 00-.174.238V7c0 1.358.275 2.666 1.057 3.86.784 1.194 2.121 2.34 4.366 3.297a.2.2 0 00.154 0c2.245-.956 3.582-2.104 4.366-3.298C13.225 9.666 13.5 8.36 13.5 7V3.48a.25.25 0 00-.174-.237l-5.25-1.68zM9 10.5a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.75a.75.75 0 10-1.5 0v3a.75.75 0 001.5 0v-3z\"></path></svg> <span data-content=\"Security\">Security</span> <include-fragment src=\"/bitcoin/bitcoin/security/overall-count\" accept=\"text/fragment+html\"></include-fragment> </a></li> <li data-view-component=\"true\" class=\"d-flex\"> <a href=\"/bitcoin/bitcoin/pulse\" data-tab-item=\"i5insights-tab\" data-selected-links=\"repo_graphs repo_contributors dependency_graph dependabot_updates pulse people community /bitcoin/bitcoin/pulse\" data-ga-click=\"Repository, Navigation click, Insights tab\" data-view-component=\"true\" class=\"UnderlineNav-item hx_underlinenav-item no-wrap js-responsive-underlinenav-item\"> <svg class=\"octicon octicon-graph UnderlineNav-octicon d-none d-sm-inline\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M1.5 1.75a.75.75 0 00-1.5 0v12.5c0 .414.336.75.75.75h14.5a.75.75 0 000-1.5H1.5V1.75zm14.28 2.53a.75.75 0 00-1.06-1.06L10 7.94 7.53 5.47a.75.75 0 00-1.06 0L3.22 8.72a.75.75 0 001.06 1.06L7 7.06l2.47 2.47a.75.75 0 001.06 0l5.25-5.25z\"></path></svg> <span data-content=\"Insights\">Insights</span> <span title=\"Not available\" data-view-component=\"true\" class=\"Counter\"></span> </a></li> </ul> <div style=\"visibility:hidden;\" data-view-component=\"true\" class=\"UnderlineNav-actions js-responsive-underlinenav-overflow position-absolute pr-3 pr-md-4 pr-lg-5 right-0\"> <details data-view-component=\"true\" class=\"details-overlay details-reset position-relative\"> <summary role=\"button\" data-view-component=\"true\"> <div class=\"UnderlineNav-item mr-0 border-0\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-kebab-horizontal\"> <path d=\"M8 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zM1.5 9a1.5 1.5 0 100-3 1.5 1.5 0 000 3zm13 0a1.5 1.5 0 100-3 1.5 1.5 0 000 3z\"></path> </svg> <span class=\"sr-only\">More</span> </div> </summary> <div data-view-component=\"true\"> <details-menu role=\"menu\" data-view-component=\"true\" class=\"dropdown-menu dropdown-menu-sw\"> <ul> <li data-menu-item=\"i0code-tab\" hidden> <a role=\"menuitem\" class=\"js-selected-navigation-item selected dropdown-item\" aria-current=\"page\" data-selected-links=\" /bitcoin/bitcoin\" href=\"/bitcoin/bitcoin\"> Code </a> </li> <li data-menu-item=\"i1issues-tab\" hidden> <a role=\"menuitem\" class=\"js-selected-navigation-item dropdown-item\" data-selected-links=\" /bitcoin/bitcoin/issues\" href=\"/bitcoin/bitcoin/issues\"> Issues </a> </li> <li data-menu-item=\"i2pull-requests-tab\" hidden> <a role=\"menuitem\" class=\"js-selected-navigation-item dropdown-item\" data-selected-links=\" /bitcoin/bitcoin/pulls\" href=\"/bitcoin/bitcoin/pulls\"> Pull requests </a> </li> <li data-menu-item=\"i3projects-tab\" hidden> <a role=\"menuitem\" class=\"js-selected-navigation-item dropdown-item\" data-selected-links=\" /bitcoin/bitcoin/projects\" href=\"/bitcoin/bitcoin/projects\"> Projects </a> </li> <li data-menu-item=\"i4security-tab\" hidden> <a role=\"menuitem\" class=\"js-selected-navigation-item dropdown-item\" data-selected-links=\" /bitcoin/bitcoin/security\" href=\"/bitcoin/bitcoin/security\"> Security </a> </li> <li data-menu-item=\"i5insights-tab\" hidden> <a role=\"menuitem\" class=\"js-selected-navigation-item dropdown-item\" data-selected-links=\" /bitcoin/bitcoin/pulse\" href=\"/bitcoin/bitcoin/pulse\"> Insights </a> </li> </ul> </details-menu></div> </details></div> </nav> </div> <div class=\"container-xl clearfix new-discussion-timeline px-3 px-md-4 px-lg-5\"> <div id=\"repo-content-pjax-container\" class=\"repository-content \" > <div> <div class=\"d-none d-lg-block mt-6 mr-3 Popover top-0 right-0 color-shadow-medium col-3\"> </div> <div data-view-component=\"true\" class=\"gutter-condensed gutter-lg flex-column flex-md-row d-flex\"> <div data-view-component=\"true\" class=\"flex-shrink-0 col-12 col-md-9 mb-4 mb-md-0\"> <div class=\"file-navigation mb-3 d-flex flex-items-start\"> <div class=\"position-relative\"> <details class=\"details-reset details-overlay mr-0 mb-0 \" id=\"branch-select-menu\"> <summary class=\"btn css-truncate\" data-hotkey=\"w\" title=\"Switch branches or tags\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-git-branch text-gray\"> <path fill-rule=\"evenodd\" d=\"M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z\"></path> </svg> <span class=\"css-truncate-target\" data-menu-button>master</span> <span class=\"dropdown-caret\"></span> </summary> <div class=\"SelectMenu\"> <div class=\"SelectMenu-modal\"> <header class=\"SelectMenu-header\"> <span class=\"SelectMenu-title\">Switch branches/tags</span> <button class=\"SelectMenu-closeButton\" type=\"button\" data-toggle-for=\"branch-select-menu\"><svg aria-label=\"Close menu\" aria-hidden=\"false\" role=\"img\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-x\"> <path fill-rule=\"evenodd\" d=\"M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z\"></path> </svg></button> </header> <input-demux data-action=\"tab-container-change:input-demux#storeInput tab-container-changed:input-demux#updateInput\"> <tab-container class=\"d-flex flex-column js-branches-tags-tabs\" style=\"min-height: 0;\"> <div class=\"SelectMenu-filter\"> <input data-target=\"input-demux.source\" id=\"context-commitish-filter-field\" class=\"SelectMenu-input form-control\" aria-owns=\"ref-list-branches\" data-controls-ref-menu-id=\"ref-list-branches\" autofocus autocomplete=\"off\" aria-label=\"Filter branches/tags\" placeholder=\"Filter branches/tags\" type=\"text\" > </div> <div class=\"SelectMenu-tabs\" role=\"tablist\" data-target=\"input-demux.control\" > <button class=\"SelectMenu-tab\" type=\"button\" role=\"tab\" aria-selected=\"true\">Branches</button> <button class=\"SelectMenu-tab\" type=\"button\" role=\"tab\">Tags</button> </div> <div role=\"tabpanel\" id=\"ref-list-branches\" data-filter-placeholder=\"Filter branches/tags\" class=\"d-flex flex-column flex-auto overflow-auto\" tabindex=\"\"> <ref-selector type=\"branch\" data-targets=\"input-demux.sinks\" data-action=\" input-entered:ref-selector#inputEntered tab-selected:ref-selector#tabSelected focus-list:ref-selector#focusFirstListMember \" query-endpoint=\"/bitcoin/bitcoin/refs\" cache-key=\"v0:1622739464.0582771\" current-committish=\"bWFzdGVy\" default-branch=\"bWFzdGVy\" name-with-owner=\"Yml0Y29pbi9iaXRjb2lu\" > <template data-target=\"ref-selector.fetchFailedTemplate\"> <div class=\"SelectMenu-message\" data-index=\"{{ index }}\">Could not load branches</div> </template> <template data-target=\"ref-selector.noMatchTemplate\"> <div class=\"SelectMenu-message\">Nothing to show</div> </template> <div data-target=\"ref-selector.listContainer\" role=\"menu\" class=\"SelectMenu-list \" style=\"max-height: 330px\"> <div class=\"SelectMenu-loading pt-3 pb-0\" aria-label=\"Menu is loading\"> <svg style=\"box-sizing: content-box; color: var(--color-icon-primary);\" viewBox=\"0 0 16 16\" fill=\"none\" data-view-component=\"true\" width=\"32\" height=\"32\" class=\"anim-rotate\"> <circle cx=\"8\" cy=\"8\" r=\"7\" stroke=\"currentColor\" stroke-opacity=\"0.25\" stroke-width=\"2\" vector-effect=\"non-scaling-stroke\" /> <path d=\"M15 8a7.002 7.002 0 00-7-7\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" vector-effect=\"non-scaling-stroke\" /> </svg> </div> </div> <template data-target=\"ref-selector.itemTemplate\"> <a href=\"https://github.com/bitcoin/bitcoin/tree/{{ urlEncodedRefName }}\" class=\"SelectMenu-item\" role=\"menuitemradio\" rel=\"nofollow\" aria-checked=\"{{ isCurrent }}\" data-index=\"{{ index }}\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-check SelectMenu-icon SelectMenu-icon--check\"> <path fill-rule=\"evenodd\" d=\"M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z\"></path> </svg> <span class=\"flex-1 css-truncate css-truncate-overflow {{ isFilteringClass }}\">{{ refName }}</span> <span hidden=\"{{ isNotDefault }}\" class=\"Label Label--secondary flex-self-start\">default</span> </a> </template> <footer class=\"SelectMenu-footer\"><a href=\"/bitcoin/bitcoin/branches\">View all branches</a></footer> </ref-selector> </div> <div role=\"tabpanel\" id=\"tags-menu\" data-filter-placeholder=\"Find a tag\" class=\"d-flex flex-column flex-auto overflow-auto\" tabindex=\"\" hidden> <ref-selector type=\"tag\" data-action=\" input-entered:ref-selector#inputEntered tab-selected:ref-selector#tabSelected focus-list:ref-selector#focusFirstListMember \" data-targets=\"input-demux.sinks\" query-endpoint=\"/bitcoin/bitcoin/refs\" cache-key=\"v0:1622739464.0582771\" current-committish=\"bWFzdGVy\" default-branch=\"bWFzdGVy\" name-with-owner=\"Yml0Y29pbi9iaXRjb2lu\" > <template data-target=\"ref-selector.fetchFailedTemplate\"> <div class=\"SelectMenu-message\" data-index=\"{{ index }}\">Could not load tags</div> </template> <template data-target=\"ref-selector.noMatchTemplate\"> <div class=\"SelectMenu-message\" data-index=\"{{ index }}\">Nothing to show</div> </template> <template data-target=\"ref-selector.itemTemplate\"> <a href=\"https://github.com/bitcoin/bitcoin/tree/{{ urlEncodedRefName }}\" class=\"SelectMenu-item\" role=\"menuitemradio\" rel=\"nofollow\" aria-checked=\"{{ isCurrent }}\" data-index=\"{{ index }}\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-check SelectMenu-icon SelectMenu-icon--check\"> <path fill-rule=\"evenodd\" d=\"M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z\"></path> </svg> <span class=\"flex-1 css-truncate css-truncate-overflow {{ isFilteringClass }}\">{{ refName }}</span> <span hidden=\"{{ isNotDefault }}\" class=\"Label Label--secondary flex-self-start\">default</span> </a> </template> <div data-target=\"ref-selector.listContainer\" role=\"menu\" class=\"SelectMenu-list\" style=\"max-height: 330px\"> <div class=\"SelectMenu-loading pt-3 pb-0\" aria-label=\"Menu is loading\"> <svg style=\"box-sizing: content-box; color: var(--color-icon-primary);\" viewBox=\"0 0 16 16\" fill=\"none\" data-view-component=\"true\" width=\"32\" height=\"32\" class=\"anim-rotate\"> <circle cx=\"8\" cy=\"8\" r=\"7\" stroke=\"currentColor\" stroke-opacity=\"0.25\" stroke-width=\"2\" vector-effect=\"non-scaling-stroke\" /> <path d=\"M15 8a7.002 7.002 0 00-7-7\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" vector-effect=\"non-scaling-stroke\" /> </svg> </div> </div> <footer class=\"SelectMenu-footer\"><a href=\"/bitcoin/bitcoin/tags\">View all tags</a></footer> </ref-selector> </div> </tab-container> </input-demux> </div> </div> </details> </div> <div class=\"flex-self-center ml-3 flex-self-stretch d-none d-lg-flex flex-items-center lh-condensed-ultra\"> <a data-pjax href=\"/bitcoin/bitcoin/branches\" class=\"Link--primary no-underline\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-git-branch text-gray\"> <path fill-rule=\"evenodd\" d=\"M11.75 2.5a.75.75 0 100 1.5.75.75 0 000-1.5zm-2.25.75a2.25 2.25 0 113 2.122V6A2.5 2.5 0 0110 8.5H6a1 1 0 00-1 1v1.128a2.251 2.251 0 11-1.5 0V5.372a2.25 2.25 0 111.5 0v1.836A2.492 2.492 0 016 7h4a1 1 0 001-1v-.628A2.25 2.25 0 019.5 3.25zM4.25 12a.75.75 0 100 1.5.75.75 0 000-1.5zM3.5 3.25a.75.75 0 111.5 0 .75.75 0 01-1.5 0z\"></path> </svg> <strong>7</strong> <span class=\"color-text-tertiary\">branches</span> </a> <a data-pjax href=\"/bitcoin/bitcoin/tags\" class=\"ml-3 Link--primary no-underline\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-tag text-gray\"> <path fill-rule=\"evenodd\" d=\"M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z\"></path> </svg> <strong>249</strong> <span class=\"color-text-tertiary\">tags</span> </a> </div> <div class=\"flex-auto\"></div> <include-fragment data-test-selector=\"overview-actions-fragment\" src=\"/bitcoin/bitcoin/overview_actions/master\"></include-fragment> <span class=\"d-none d-md-flex ml-2\"> <get-repo> <details class=\"position-relative details-overlay details-reset\" data-action=\"toggle:get-repo#onDetailsToggle\"> <summary class=\"btn btn-primary\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository.click&quot;,&quot;payload&quot;:{&quot;repository_id&quot;:1181927,&quot;target&quot;:&quot;CLONE_OR_DOWNLOAD_BUTTON&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"bbdb79cf43ea50486634c8b48b3c6dec3de5e2a011e94b2466373cab3be203fe\"> <svg class=\"octicon octicon-download mr-1\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.47 10.78a.75.75 0 001.06 0l3.75-3.75a.75.75 0 00-1.06-1.06L8.75 8.44V1.75a.75.75 0 00-1.5 0v6.69L4.78 5.97a.75.75 0 00-1.06 1.06l3.75 3.75zM3.75 13a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5z\"></path></svg> Code <span class=\"dropdown-caret\"></span> </summary> <div class=\"position-relative\"> <div class=\"dropdown-menu dropdown-menu-sw p-0\" style=\"top:6px;width:378px;\"> <div data-target=\"get-repo.modal\"> <div class=\"border-bottom p-3\"> <a class=\"Link--muted float-right tooltipped tooltipped-s\" href=\"https://docs.github.com/articles/which-remote-url-should-i-use\" target=\"_blank\" aria-label=\"Which remote URL should I use?\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-question\"> <path fill-rule=\"evenodd\" d=\"M8 1.5a6.5 6.5 0 100 13 6.5 6.5 0 000-13zM0 8a8 8 0 1116 0A8 8 0 010 8zm9 3a1 1 0 11-2 0 1 1 0 012 0zM6.92 6.085c.081-.16.19-.299.34-.398.145-.097.371-.187.74-.187.28 0 .553.087.738.225A.613.613 0 019 6.25c0 .177-.04.264-.077.318a.956.956 0 01-.277.245c-.076.051-.158.1-.258.161l-.007.004a7.728 7.728 0 00-.313.195 2.416 2.416 0 00-.692.661.75.75 0 001.248.832.956.956 0 01.276-.245 6.3 6.3 0 01.26-.16l.006-.004c.093-.057.204-.123.313-.195.222-.149.487-.355.692-.662.214-.32.329-.702.329-1.15 0-.76-.36-1.348-.863-1.725A2.76 2.76 0 008 4c-.631 0-1.155.16-1.572.438-.413.276-.68.638-.849.977a.75.75 0 101.342.67z\"></path> </svg> </a> <div class=\"text-bold\"> <svg class=\"octicon octicon-terminal mr-3\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M0 2.75C0 1.784.784 1 1.75 1h12.5c.966 0 1.75.784 1.75 1.75v10.5A1.75 1.75 0 0114.25 15H1.75A1.75 1.75 0 010 13.25V2.75zm1.75-.25a.25.25 0 00-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 00.25-.25V2.75a.25.25 0 00-.25-.25H1.75zM7.25 8a.75.75 0 01-.22.53l-2.25 2.25a.75.75 0 11-1.06-1.06L5.44 8 3.72 6.28a.75.75 0 111.06-1.06l2.25 2.25c.141.14.22.331.22.53zm1.5 1.5a.75.75 0 000 1.5h3a.75.75 0 000-1.5h-3z\"></path></svg> Clone </div> <tab-container> <div class=\"UnderlineNav my-2 box-shadow-none\"> <div class=\"UnderlineNav-body\" role=\"tablist\"> <button name=\"button\" type=\"button\" role=\"tab\" class=\"UnderlineNav-item lh-default f6 py-0 px-0 mr-2 position-relative\" aria-selected=\"true\" data-hydro-click=\"{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;USE_HTTPS&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"aff095a749269da6c57f6a53edd756af7c8101637235dc8c402443f5979424cb\"> HTTPS </button> <button name=\"button\" type=\"button\" role=\"tab\" class=\"UnderlineNav-item lh-default f6 py-0 px-0 mr-2 position-relative\" data-hydro-click=\"{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;USE_GH_CLI&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"884f12eb9cdb96bbcdc2fd55c68335c4a5e90437235527bdfc4f41c2fb548abe\"> GitHub CLI </button> </div> </div> <div role=\"tabpanel\"> <div class=\"input-group\"> <input type=\"text\" class=\"form-control input-monospace input-sm color-bg-secondary\" data-autoselect value=\"https://github.com/bitcoin/bitcoin.git\" aria-label=\"https://github.com/bitcoin/bitcoin.git\" readonly> <div class=\"input-group-button\"> <clipboard-copy value=\"https://github.com/bitcoin/bitcoin.git\" aria-label=\"Copy to clipboard\" class=\"btn btn-sm js-clipboard-copy tooltipped-no-delay ClipboardButton\" data-copy-feedback=\"Copied!\" data-tooltip-direction=\"n\" data-hydro-click=\"{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;COPY_URL&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"11a44ab28a7c55a003054a9fe2eadae27bc9a9aacfad67333e356bb666c6b5fa\"><svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-clippy js-clipboard-clippy-icon d-inline-block\"> <path fill-rule=\"evenodd\" d=\"M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z\"></path> </svg><svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-check js-clipboard-check-icon color-text-success d-inline-block d-sm-none\"> <path fill-rule=\"evenodd\" d=\"M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z\"></path> </svg></clipboard-copy> </div> </div> <p class=\"mt-2 mb-0 f6 color-text-secondary\"> Use Git or checkout with SVN using the web URL. </p> </div> <div role=\"tabpanel\" hidden> <div class=\"input-group\"> <input type=\"text\" class=\"form-control input-monospace input-sm color-bg-secondary\" data-autoselect value=\"gh repo clone bitcoin/bitcoin\" aria-label=\"gh repo clone bitcoin/bitcoin\" readonly> <div class=\"input-group-button\"> <clipboard-copy value=\"gh repo clone bitcoin/bitcoin\" aria-label=\"Copy to clipboard\" class=\"btn btn-sm js-clipboard-copy tooltipped-no-delay ClipboardButton\" data-copy-feedback=\"Copied!\" data-tooltip-direction=\"n\" data-hydro-click=\"{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;COPY_URL&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"11a44ab28a7c55a003054a9fe2eadae27bc9a9aacfad67333e356bb666c6b5fa\"><svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-clippy js-clipboard-clippy-icon d-inline-block\"> <path fill-rule=\"evenodd\" d=\"M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z\"></path> </svg><svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-check js-clipboard-check-icon color-text-success d-inline-block d-sm-none\"> <path fill-rule=\"evenodd\" d=\"M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z\"></path> </svg></clipboard-copy> </div> </div> <p class=\"mt-2 mb-0 f6 color-text-secondary\"> Work fast with our official CLI. <a href=\"https://cli.github.com\" target=\"_blank\">Learn more</a>. </p> </div> </tab-container> </div> <ul class=\"list-style-none\"> <li data-platforms=\"windows,mac\" class=\"Box-row Box-row--hover-gray p-0 rounded-0 mt-0 js-remove-unless-platform\"> <a class=\"d-flex flex-items-center color-text-primary text-bold no-underline p-3\" data-hydro-click=\"{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;OPEN_IN_DESKTOP&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"a663945ba8f8553bce164a2727caaac45f338f30584bcf14df89b29e4ae6f928\" data-action=\"click:get-repo#showDownloadMessage\" href=\"https://desktop.github.com\"> <svg class=\"octicon octicon-desktop-download mr-3\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path d=\"M4.927 5.427l2.896 2.896a.25.25 0 00.354 0l2.896-2.896A.25.25 0 0010.896 5H8.75V.75a.75.75 0 10-1.5 0V5H5.104a.25.25 0 00-.177.427z\"></path><path d=\"M1.573 2.573a.25.25 0 00-.073.177v7.5a.25.25 0 00.25.25h12.5a.25.25 0 00.25-.25v-7.5a.25.25 0 00-.25-.25h-3a.75.75 0 110-1.5h3A1.75 1.75 0 0116 2.75v7.5A1.75 1.75 0 0114.25 12h-3.727c.099 1.041.52 1.872 1.292 2.757A.75.75 0 0111.25 16h-6.5a.75.75 0 01-.565-1.243c.772-.885 1.192-1.716 1.292-2.757H1.75A1.75 1.75 0 010 10.25v-7.5A1.75 1.75 0 011.75 1h3a.75.75 0 010 1.5h-3a.25.25 0 00-.177.073zM6.982 12a5.72 5.72 0 01-.765 2.5h3.566a5.72 5.72 0 01-.765-2.5H6.982z\"></path></svg> Open with GitHub Desktop </a> </li> <li class=\"Box-row Box-row--hover-gray p-0\"> <a class=\"d-flex flex-items-center color-text-primary text-bold no-underline p-3\" rel=\"nofollow\" data-hydro-click=\"{&quot;event_type&quot;:&quot;clone_or_download.click&quot;,&quot;payload&quot;:{&quot;feature_clicked&quot;:&quot;DOWNLOAD_ZIP&quot;,&quot;git_repository_type&quot;:&quot;REPOSITORY&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"dbe8664f4d5f36fc8f0f5e963aea82428cb256241412e67d2585a60c5bd64300\" data-ga-click=\"Repository, download zip, location:repo overview\" data-open-app=\"link\" href=\"/bitcoin/bitcoin/archive/refs/heads/master.zip\"> <svg class=\"octicon octicon-file-zip mr-3\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M3.5 1.75a.25.25 0 01.25-.25h3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h2.086a.25.25 0 01.177.073l2.914 2.914a.25.25 0 01.073.177v8.586a.25.25 0 01-.25.25h-.5a.75.75 0 000 1.5h.5A1.75 1.75 0 0014 13.25V4.664c0-.464-.184-.909-.513-1.237L10.573.513A1.75 1.75 0 009.336 0H3.75A1.75 1.75 0 002 1.75v11.5c0 .649.353 1.214.874 1.515a.75.75 0 10.752-1.298.25.25 0 01-.126-.217V1.75zM8.75 3a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM6 5.25a.75.75 0 01.75-.75h.5a.75.75 0 010 1.5h-.5A.75.75 0 016 5.25zm2 1.5A.75.75 0 018.75 6h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 6.75zm-1.25.75a.75.75 0 000 1.5h.5a.75.75 0 000-1.5h-.5zM8 9.75A.75.75 0 018.75 9h.5a.75.75 0 010 1.5h-.5A.75.75 0 018 9.75zm-.75.75a1.75 1.75 0 00-1.75 1.75v3c0 .414.336.75.75.75h2.5a.75.75 0 00.75-.75v-3a1.75 1.75 0 00-1.75-1.75h-.5zM7 12.25a.25.25 0 01.25-.25h.5a.25.25 0 01.25.25v2.25H7v-2.25z\"></path></svg> Download ZIP </a> </li> </ul> </div> <div class=\"p-3\" data-targets=\"get-repo.platforms\" data-platform=\"mac\" hidden> <h4 class=\"lh-condensed mb-3\">Launching GitHub Desktop<span class=\"AnimatedEllipsis\"></span></h4> <p class=\"color-text-secondary\">If nothing happens, <a href=\"https://desktop.github.com/\">download GitHub Desktop</a> and try again.</p> <button type=\"button\" class=\"btn-link\" data-action=\"click:get-repo#onDetailsToggle\">Go back</button> </div> <div class=\"p-3\" data-targets=\"get-repo.platforms\" data-platform=\"windows\" hidden> <h4 class=\"lh-condensed mb-3\">Launching GitHub Desktop<span class=\"AnimatedEllipsis\"></span></h4> <p class=\"color-text-secondary\">If nothing happens, <a href=\"https://desktop.github.com/\">download GitHub Desktop</a> and try again.</p> <button type=\"button\" class=\"btn-link\" data-action=\"click:get-repo#onDetailsToggle\">Go back</button> </div> <div class=\"p-3\" data-targets=\"get-repo.platforms\" data-platform=\"xcode\" hidden> <h4 class=\"lh-condensed mb-3\">Launching Xcode<span class=\"AnimatedEllipsis\"></span></h4> <p class=\"color-text-secondary\">If nothing happens, <a href=\"https://developer.apple.com/xcode/\">download Xcode</a> and try again.</p> <button type=\"button\" class=\"btn-link\" data-action=\"click:get-repo#onDetailsToggle\">Go back</button> </div> <div class=\"p-3 \" data-targets=\"get-repo.platforms\" data-target=\"new-codespace.loadingVscode prefetch-pane.loadingVscode\" data-platform=\"vscode\" hidden> <poll-include-fragment data-target=\"get-repo.vscodePoller new-codespace.vscodePoller prefetch-pane.vscodePoller\"> <h4 class=\"lh-condensed mb-3\">Launching Visual Studio Code<span class=\"AnimatedEllipsis\" data-hide-on-error></span></h4> <p class=\"color-text-secondary\" data-hide-on-error>Your codespace will open once ready.</p> <p class=\"color-text-secondary\" data-show-on-error hidden>There was a problem preparing your codespace, please try again.</p> </poll-include-fragment> </div> </div> </div> </details> </get-repo> </span> </div> <div class=\"Box mb-3\"> <div class=\"Box-header Box-header--blue position-relative\"> <h2 class=\"sr-only\">Latest commit</h2> <div class=\"js-details-container Details d-flex rounded-top-1 flex-items-center flex-wrap\" data-issue-and-pr-hovercards-enabled> <div class=\"flex-shrink-0 ml-n1 mr-n1 mt-n1 mb-n1 hx_avatar_stack_commit\" > <div class=\"AvatarStack flex-self-start \" > <div class=\"AvatarStack-body\" aria-label=\"MarcoFalke\" > <a class=\"avatar avatar-user\" style=\"width:24px;height:24px;\" data-skip-pjax=\"true\" data-test-selector=\"commits-avatar-stack-avatar-link\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/MarcoFalke/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/MarcoFalke\"> <img data-test-selector=\"commits-avatar-stack-avatar-image\" src=\"https://avatars.githubusercontent.com/u/6399679?s=48&amp;v=4\" width=\"24\" height=\"24\" alt=\"@MarcoFalke\" class=\" avatar-user\" /> </a> </div> </div> </div> <div class=\"flex-1 d-flex flex-items-center ml-3 min-width-0\"> <div class=\"css-truncate css-truncate-overflow color-text-secondary\" > <a href=\"/bitcoin/bitcoin/commits?author=MarcoFalke\" class=\"commit-author user-mention\" title=\"View all commits by MarcoFalke\">MarcoFalke</a> <span class=\"d-none d-sm-inline\"> <a data-pjax=\"true\" data-test-selector=\"commit-tease-commit-message\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--primary markdown-title\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"789282167\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/20966\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/20966/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/20966\">#20966</a><a data-pjax=\"true\" data-test-selector=\"commit-tease-commit-message\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--primary markdown-title\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">: banman: save the banlist in a JSON format on disk</a> </span> </div> <span class=\"hidden-text-expander ml-2 d-inline-block \" > <button type=\"button\" class=\"color-text-primary ellipsis-expander js-details-target\" aria-expanded=\"false\" > &hellip; </button> </span> <div class=\"d-flex flex-auto flex-justify-end ml-3 flex-items-baseline\"> <include-fragment accept=\"text/fragment+html\" src=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d/rollup?direction=sw\" class=\"d-inline\"></include-fragment> <a href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\" class=\"f6 Link--secondary text-mono ml-2 d-none d-lg-inline\" data-pjax > d6e0d78 </a> <a href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\" class=\"Link--secondary ml-2\" data-pjax > <relative-time datetime=\"2021-06-23T08:01:56Z\" class=\"no-wrap\">Jun 23, 2021</relative-time> </a> </div> </div> <div class=\"pl-0 pl-md-5 flex-order-1 width-full Details-content--hidden\"> <div class=\"mt-2\"> <a data-pjax=\"true\" data-test-selector=\"commit-tease-commit-message\" class=\"Link--primary text-bold\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"789282167\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/20966\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/20966/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/20966\">#20966</a><a data-pjax=\"true\" data-test-selector=\"commit-tease-commit-message\" class=\"Link--primary text-bold\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">: banman: save the banlist in a JSON format on disk</a> </div> <pre class=\"mt-2 text-mono color-text-secondary text-small ws-pre-wrap\"><a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a\"><tt>bb719a0</tt></a> style: remove () from assert in rpc_setban.py (Vasil Dimov) <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/24b10ebda301548b8ff4b0c73fefc367ad5dc22b/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/24b10ebda301548b8ff4b0c73fefc367ad5dc22b\"><tt>24b10eb</tt></a> doc: fix grammar in doc/files.md (Vasil Dimov) <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/dd4e957dcdfc971a4a971995ff2db9fb787d23c3/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/dd4e957dcdfc971a4a971995ff2db9fb787d23c3\"><tt>dd4e957</tt></a> test: ensure banlist can be read from disk after restart (Vasil Dimov) <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/d197977ae2076903ed12ab7616a7f93e88be02e1/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/d197977ae2076903ed12ab7616a7f93e88be02e1\"><tt>d197977</tt></a> banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"783522047\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/20904\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/20904/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/20904\">#20904</a> <span class=\"issue-keyword tooltipped tooltipped-se\" aria-label=\"This commit closes issue #19748.\">Resolves</span> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"680274656\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/19748\" data-hovercard-type=\"issue\" data-hovercard-url=\"/bitcoin/bitcoin/issues/19748/hovercard\" href=\"https://github.com/bitcoin/bitcoin/issues/19748\">#19748</a> ACKs for top commit: jonatack: Code review re-ACK <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a\"><tt>bb719a0</tt></a> per `git range-diff <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/6a67366fdc3e1d383fe7cbfa209d7e85f0d96638/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/6a67366fdc3e1d383fe7cbfa209d7e85f0d96638\"><tt>6a67366</tt></a> <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/4b52c7234f3c2e3067d7d6c6fd7ebf2b96bb8a45/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/4b52c7234f3c2e3067d7d6c6fd7ebf2b96bb8a45\"><tt>4b52c72</tt></a> <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a\"><tt>bb719a0</tt></a>` achow101: Code Review ACK <a class=\"commit-link\" data-hovercard-type=\"commit\" data-hovercard-url=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a/hovercard\" href=\"https://github.com/bitcoin/bitcoin/commit/bb719a08db173a753984a04534de6f148b87b17a\"><tt>bb719a0</tt></a> Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6</pre> <div class=\"d-flex flex-items-center\"> <code class=\"border d-lg-none mt-2 px-1 rounded-1\">d6e0d78</code> </div> </div> <div class=\"flex-shrink-0\"> <h2 class=\"sr-only\">Git stats</h2> <ul class=\"list-style-none d-flex\"> <li class=\"ml-0 ml-md-3\"> <a data-pjax href=\"/bitcoin/bitcoin/commits/master\" class=\"pl-3 pr-3 py-3 p-md-0 mt-n3 mb-n3 mr-n3 m-md-0 Link--primary no-underline no-wrap\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-history text-gray\"> <path fill-rule=\"evenodd\" d=\"M1.643 3.143L.427 1.927A.25.25 0 000 2.104V5.75c0 .138.112.25.25.25h3.646a.25.25 0 00.177-.427L2.715 4.215a6.5 6.5 0 11-1.18 4.458.75.75 0 10-1.493.154 8.001 8.001 0 101.6-5.684zM7.75 4a.75.75 0 01.75.75v2.992l2.028.812a.75.75 0 01-.557 1.392l-2.5-1A.75.75 0 017 8.25v-3.5A.75.75 0 017.75 4z\"></path> </svg> <span class=\"d-none d-sm-inline\"> <strong>29,619</strong> <span aria-label=\"Commits on master\" class=\"color-text-secondary d-none d-lg-inline\"> commits </span> </span> </a> </li> </ul> </div> </div> </div> <h2 id=\"files\" class=\"sr-only\">Files</h2> <a class=\"d-none js-permalink-shortcut\" data-hotkey=\"y\" href=\"/bitcoin/bitcoin/tree/d6e0d78c31557660274ef53cac912c468eecbe2d\">Permalink</a> <div data-view-component=\"true\" class=\"include-fragment-error flash flash-error flash-full py-2\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-alert\"> <path fill-rule=\"evenodd\" d=\"M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z\"></path> </svg> Failed to load latest commit information. </div> <div class=\"js-details-container Details\"> <div role=\"grid\" aria-labelledby=\"files\" class=\"Details-content--hidden-not-important js-navigation-container js-active-navigation-container d-md-block\" data-pjax> <div class=\"sr-only\" role=\"row\"> <div role=\"columnheader\">Type</div> <div role=\"columnheader\">Name</div> <div role=\"columnheader\" class=\"d-none d-md-block\">Latest commit message</div> <div role=\"columnheader\">Commit time</div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".github\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/.github\">.github</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"doc: Remove label from good first issue template\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/fa30d5282cb07b6de0160d7df8b649332db97dde\">doc: Remove label from good first issue template</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2020-08-24T07:31:24Z\" data-view-component=\"true\" class=\"no-wrap\">Aug 24, 2020</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".tx\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/.tx\">.tx</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"qt: Bump transifex slug for 22.x Opening the 22.x translations early because of experimentation with the new xliff translations format. In this context, change file_filter to `xlf` as well as the files pulled with `tx pull` are that format now (the setting only affects the naming not the format of the files). Tree-SHA512: e0c18aa5e6cbd4428d24324fee8e5761b70dae51d0236277577aded719798c6a32fc81c0598f280321f2816629e33a334f61f9e7f6180c4074abfda6550cefbe\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/417305991a0573484f4aa3820103d8b991cb8f81\">qt: Bump transifex slug for 22.x</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-04-21T11:46:41Z\" data-view-component=\"true\" class=\"no-wrap\">Apr 21, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"This path skips through empty directories\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/build-aux/m4\"><span class=\"color-text-tertiary\">build-aux/</span>m4</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"build, qt: Fix libraries linking order for Linux hosts This change fixes configuring with Qt on Alpine Linux.\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/a8bd5ea01720e5639cabdc9897cf9cf7c02c47c6\">build, qt: Fix libraries linking order for Linux hosts</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-06T20:25:07Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 6, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"build_msvc\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/build_msvc\">build_msvc</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#22230: build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe 9edd713c184bd6b92ff7862d0df8f1a89062e9d3 build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe (Hennadii Stepanov) Pull request description: On master (6f3fbc062f97183f19a8551177371cc74a33351d), running `bitcoin-qt.exe`, which was built with MSVC, causes a terminal window open along with the GUI. This PR fixes such behavior. See Microsoft [docs](https://docs.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-160). It is still possible to use the `-printtoconsole` option for debug builds. ACKs for top commit: sipsorcery: tACK 9edd713c184bd6b92ff7862d0df8f1a89062e9d3. Tree-SHA512: 02f2874b13e484f98344f6a7e3b01fa82a78a39865787c77bd674ead22a84a7f98a1849ccad26bd2b8c8603b3e29dcc1633b0ad731ce7d61be2d6b1f9584839c\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/de5512e28df220990ad123b914167aadd6f50979\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"919577035\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/22230\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/22230/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/22230\">#22230</a><a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#22230: build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe 9edd713c184bd6b92ff7862d0df8f1a89062e9d3 build: Fix MSVC linker /SubSystem option for bitcoin-qt.exe (Hennadii Stepanov) Pull request description: On master (6f3fbc062f97183f19a8551177371cc74a33351d), running `bitcoin-qt.exe`, which was built with MSVC, causes a terminal window open along with the GUI. This PR fixes such behavior. See Microsoft [docs](https://docs.microsoft.com/en-us/cpp/build/reference/subsystem-specify-subsystem?view=msvc-160). It is still possible to use the `-printtoconsole` option for debug builds. ACKs for top commit: sipsorcery: tACK 9edd713c184bd6b92ff7862d0df8f1a89062e9d3. Tree-SHA512: 02f2874b13e484f98344f6a7e3b01fa82a78a39865787c77bd674ead22a84a7f98a1849ccad26bd2b8c8603b3e29dcc1633b0ad731ce7d61be2d6b1f9584839c\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/de5512e28df220990ad123b914167aadd6f50979\">: build: Fix MSVC linker /SubSystem option for bitcoin-qt…</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-14T02:06:55Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 14, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"ci\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/ci\">ci</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"build: enable external signer by default\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/5be90c907eba0a38019c7d9826623d5d5f567c66\">build: enable external signer by default</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-16T08:48:57Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 16, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"contrib\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/contrib\">contrib</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#22244: devtools: Correctly extract symbol versions in symbol-check e8cd3700eeb27437f5ea435869c9d61214285fdd devtools: Integrate ARCH_MIN_GLIBC_VER table into MAX_VERSIONS in symbol-check.py (W. J. van der Laan) a33381acf5ae2b43616fffaf26b1c8962e8ef0bb devtools: Add xkb version to symbol-check (W. J. van der Laan) 19e598bab0a1cb5ad93321eb9fa25d1a58d5e276 devtools: Fix verneed section parsing in pixie (W. J. van der Laan) Pull request description: I misunderstood the ELF specification for version symbols (verneed): The `vn_aux` pointer is relative to the main verneed record, not the start of the section. This caused many symbols to not be versioned properly in the return value of `elf.dyn_symbols`. This was discovered in #21454. Fix it by correcting the offset computation. - xkb versions symbols (using the prefix `V`), as this library is used by bitcoin-qt, add it to the valid versions in `symbol-check.py` This unfortunately brings to light some symbols that have been introduced since and weren\'t caught (from a gitian compile of master): ``` bitcoin-cli: symbol getrandom from unsupported version GLIBC_2.25 bitcoin-cli: failed IMPORTED_SYMBOLS bitcoind: symbol getrandom from unsupported version GLIBC_2.25 bitcoind: symbol log from unsupported version GLIBC_2.29 bitcoind: symbol fcntl64 from unsupported version GLIBC_2.28 bitcoind: symbol pow from unsupported version GLIBC_2.29 bitcoind: symbol exp from unsupported version GLIBC_2.29 bitcoind: failed IMPORTED_SYMBOLS bitcoin-qt: symbol exp from unsupported version GLIBC_2.29 bitcoin-qt: symbol fcntl64 from unsupported version GLIBC_2.28 bitcoin-qt: symbol log from unsupported version GLIBC_2.29 bitcoin-qt: symbol pow from unsupported version GLIBC_2.29 bitcoin-qt: symbol statx from unsupported version GLIBC_2.28 bitcoin-qt: symbol getrandom from unsupported version GLIBC_2.25 bitcoin-qt: symbol renameat2 from unsupported version GLIBC_2.28 bitcoin-qt: symbol getentropy from unsupported version GLIBC_2.25 bitcoin-qt: failed IMPORTED_SYMBOLS bitcoin-wallet: symbol exp from unsupported version GLIBC_2.29 bitcoin-wallet: symbol log from unsupported version GLIBC_2.29 bitcoin-wallet: symbol fcntl64 from unsupported version GLIBC_2.28 bitcoin-wallet: failed IMPORTED_SYMBOLS test_bitcoin: symbol getrandom from unsupported version GLIBC_2.25 test_bitcoin: symbol log from unsupported version GLIBC_2.29 test_bitcoin: symbol fcntl64 from unsupported version GLIBC_2.28 test_bitcoin: symbol pow from unsupported version GLIBC_2.29 test_bitcoin: symbol exp from unsupported version GLIBC_2.29 test_bitcoin: failed IMPORTED_SYMBOLS ``` ACKs for top commit: hebasto: ACK e8cd3700eeb27437f5ea435869c9d61214285fdd Tree-SHA512: 8c15e3478eb642f01a1ddaadef03f80583f088f9fa8e3bf171ce16b0ec05ffb4675ec147d7ffc6a4360637ed47fca517c6ca2bac7bb30d794c03783cfb964b79\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/a305a687e70cfe1bfe5e57161fa9a084b290cd7b\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"920674557\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/22244\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/22244/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/22244\">#22244</a><a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#22244: devtools: Correctly extract symbol versions in symbol-check e8cd3700eeb27437f5ea435869c9d61214285fdd devtools: Integrate ARCH_MIN_GLIBC_VER table into MAX_VERSIONS in symbol-check.py (W. J. van der Laan) a33381acf5ae2b43616fffaf26b1c8962e8ef0bb devtools: Add xkb version to symbol-check (W. J. van der Laan) 19e598bab0a1cb5ad93321eb9fa25d1a58d5e276 devtools: Fix verneed section parsing in pixie (W. J. van der Laan) Pull request description: I misunderstood the ELF specification for version symbols (verneed): The `vn_aux` pointer is relative to the main verneed record, not the start of the section. This caused many symbols to not be versioned properly in the return value of `elf.dyn_symbols`. This was discovered in #21454. Fix it by correcting the offset computation. - xkb versions symbols (using the prefix `V`), as this library is used by bitcoin-qt, add it to the valid versions in `symbol-check.py` This unfortunately brings to light some symbols that have been introduced since and weren\'t caught (from a gitian compile of master): ``` bitcoin-cli: symbol getrandom from unsupported version GLIBC_2.25 bitcoin-cli: failed IMPORTED_SYMBOLS bitcoind: symbol getrandom from unsupported version GLIBC_2.25 bitcoind: symbol log from unsupported version GLIBC_2.29 bitcoind: symbol fcntl64 from unsupported version GLIBC_2.28 bitcoind: symbol pow from unsupported version GLIBC_2.29 bitcoind: symbol exp from unsupported version GLIBC_2.29 bitcoind: failed IMPORTED_SYMBOLS bitcoin-qt: symbol exp from unsupported version GLIBC_2.29 bitcoin-qt: symbol fcntl64 from unsupported version GLIBC_2.28 bitcoin-qt: symbol log from unsupported version GLIBC_2.29 bitcoin-qt: symbol pow from unsupported version GLIBC_2.29 bitcoin-qt: symbol statx from unsupported version GLIBC_2.28 bitcoin-qt: symbol getrandom from unsupported version GLIBC_2.25 bitcoin-qt: symbol renameat2 from unsupported version GLIBC_2.28 bitcoin-qt: symbol getentropy from unsupported version GLIBC_2.25 bitcoin-qt: failed IMPORTED_SYMBOLS bitcoin-wallet: symbol exp from unsupported version GLIBC_2.29 bitcoin-wallet: symbol log from unsupported version GLIBC_2.29 bitcoin-wallet: symbol fcntl64 from unsupported version GLIBC_2.28 bitcoin-wallet: failed IMPORTED_SYMBOLS test_bitcoin: symbol getrandom from unsupported version GLIBC_2.25 test_bitcoin: symbol log from unsupported version GLIBC_2.29 test_bitcoin: symbol fcntl64 from unsupported version GLIBC_2.28 test_bitcoin: symbol pow from unsupported version GLIBC_2.29 test_bitcoin: symbol exp from unsupported version GLIBC_2.29 test_bitcoin: failed IMPORTED_SYMBOLS ``` ACKs for top commit: hebasto: ACK e8cd3700eeb27437f5ea435869c9d61214285fdd Tree-SHA512: 8c15e3478eb642f01a1ddaadef03f80583f088f9fa8e3bf171ce16b0ec05ffb4675ec147d7ffc6a4360637ed47fca517c6ca2bac7bb30d794c03783cfb964b79\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/a305a687e70cfe1bfe5e57161fa9a084b290cd7b\">: devtools: Correctly extract symbol versions in symbol-c…</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-21T05:58:12Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 21, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"depends\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/depends\">depends</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"build, qt: Fix compiling qt package in depends with GCC 11\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d1d1cc983146ece950430da78ebae6f502913a53\">build, qt: Fix compiling qt package in depends with GCC 11</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-08T01:16:36Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 8, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"doc\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/doc\">doc</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"789282167\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/20966\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/20966/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/20966\">#20966</a><a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">: banman: save the banlist in a JSON format on disk</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-23T08:01:56Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 23, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"share\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/share\">share</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"doc: add maxuploadtarget to bitcoin.conf example Introduce the maxuploadtarget option to the example bitcoin.conf file. This adds visibility for this option which is useful to those looking to configure bandwidth usage.\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/947f9734daab4e47c0abdc6ef7d52812102ecb6b\">doc: add maxuploadtarget to bitcoin.conf example</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-05-28T16:53:17Z\" data-view-component=\"true\" class=\"no-wrap\">May 28, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"src\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/src\">src</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"789282167\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/20966\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/20966/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/20966\">#20966</a><a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">: banman: save the banlist in a JSON format on disk</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-23T08:01:56Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 23, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"Directory\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file-directory hx_color-icon-directory\"> <path fill-rule=\"evenodd\" d=\"M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3h-6.5a.25.25 0 01-.2-.1l-.9-1.2c-.33-.44-.85-.7-1.4-.7h-3.5z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"test\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/tree/master/test\">test</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"789282167\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/20966\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/20966/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/20966\">#20966</a><a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#20966: banman: save the banlist in a JSON format on disk bb719a08db173a753984a04534de6f148b87b17a style: remove () from assert in rpc_setban.py (Vasil Dimov) 24b10ebda301548b8ff4b0c73fefc367ad5dc22b doc: fix grammar in doc/files.md (Vasil Dimov) dd4e957dcdfc971a4a971995ff2db9fb787d23c3 test: ensure banlist can be read from disk after restart (Vasil Dimov) d197977ae2076903ed12ab7616a7f93e88be02e1 banman: save the banlist in a JSON format on disk (Vasil Dimov) Pull request description: Save the banlist in `banlist.json` instead of `banlist.dat`. This makes it possible to store Tor v3 entries in the banlist on disk (and any other addresses that cannot be serialized in addrv1 format). Only read `banlist.dat` if it exists and `banlist.json` does not exist (first start after an upgrade). Supersedes https://github.com/bitcoin/bitcoin/pull/20904 Resolves https://github.com/bitcoin/bitcoin/issues/19748 ACKs for top commit: jonatack: Code review re-ACK bb719a08db173a753984a04534de6f148b87b17a per `git range-diff 6a67366 4b52c72 bb719a0` achow101: Code Review ACK bb719a08db173a753984a04534de6f148b87b17a Tree-SHA512: fc135c3a1fe20bcf5d008ce6bea251b4135e56c78bf8f750b4bd8144c095b81ffe165133cdc7e4715875eec7e7c4e13ad9f5d2450b21102af063d7c8abf716b6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/d6e0d78c31557660274ef53cac912c468eecbe2d\">: banman: save the banlist in a JSON format on disk</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-23T08:01:56Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 23, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".appveyor.yml\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.appveyor.yml\">.appveyor.yml</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Switch Appveyor CI to VS2019 stable image The current appveyor config is using the VS2019 preview image so the latest prebuilt Qt5.12.11 binaries can be used, see #22224. Appveyor updated the Visual Studio 2019 image to msbuild v16.10.1 on 14th of June. This is the version used to build the latest Qt binaries and removes the need to use the Appveyor VS2019 preview image.\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/aab7fd0f8ddb34437a63d636170f5051aae285b4\">Switch Appveyor CI to VS2019 stable image</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-14T19:35:00Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 14, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".cirrus.yml\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.cirrus.yml\">.cirrus.yml</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"ci: Bump macOS image to big-sur-xcode-12.5 This also removes the &quot;brew update&quot; added in commit b7381552cd4f965c45f1560d9cfc2fb09dbfcc1d.\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/faa8dfd6a1fcd4df3624aabb3ff08c1f2be198e7\">ci: Bump macOS image to big-sur-xcode-12.5</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-02T08:03:38Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 2, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".editorconfig\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.editorconfig\">.editorconfig</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Add EditorConfig file.\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/7a135d57b2ac17477b25d5046a3bec57eac3ab30\">Add EditorConfig file.</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-02-10T07:00:06Z\" data-view-component=\"true\" class=\"no-wrap\">Feb 10, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".gitattributes\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.gitattributes\">.gitattributes</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Separate protocol versioning from clientversion\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/71697f97d3f9512f0af934070690c14f1c0d95ea\">Separate protocol versioning from clientversion</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2014-10-29T04:24:40Z\" data-view-component=\"true\" class=\"no-wrap\">Oct 29, 2014</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".gitignore\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.gitignore\">.gitignore</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"build: add *~ to .gitignore Homebrew autoconf version 2.7.1 introduces configure~ as a build artifact. Co-authored-by: Hennadii Stepanov &lt;32963518+hebasto@users.noreply.github.com&gt;\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/bc4538806e3c53e7821e01d5db896f65dd3358ad\">build: add *~ to .gitignore</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-05-12T16:10:47Z\" data-view-component=\"true\" class=\"no-wrap\">May 12, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".python-version\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.python-version\">.python-version</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Bump minimum python version to 3.6\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/8ae9d314e9af7bcce1e8bc52f0317b9d565109bf\">Bump minimum python version to 3.6</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2020-11-09T07:53:47Z\" data-view-component=\"true\" class=\"no-wrap\">Nov 9, 2020</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\".style.yapf\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/.style.yapf\">.style.yapf</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"test: .style.yapf: Set column_limit=160\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/1111f0718acea42954600a4dbd553ac40aae797f\">test: .style.yapf: Set column_limit=160</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2019-03-04T23:28:13Z\" data-view-component=\"true\" class=\"no-wrap\">Mar 4, 2019</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"CONTRIBUTING.md\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/CONTRIBUTING.md\">CONTRIBUTING.md</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"doc: Fix external links (IRC, ...)\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/9999e4c64b219ffec5105442047daf2b46be0700\">doc: Fix external links (IRC, ...)</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-05-31T15:27:57Z\" data-view-component=\"true\" class=\"no-wrap\">May 31, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"COPYING\" data-pjax=\"#repo-content-pjax-container\" itemprop=\"license\" href=\"/bitcoin/bitcoin/blob/master/COPYING\">COPYING</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"doc: Update license year range to 2021\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/ccc8d5513fb2228eabc25b105f7a0498f8453885\">doc: Update license year range to 2021</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2020-12-30T15:24:47Z\" data-view-component=\"true\" class=\"no-wrap\">Dec 30, 2020</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"INSTALL.md\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/INSTALL.md\">INSTALL.md</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Update INSTALL landing redirection notice for build instructions.\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/2920be2a6994cfbffd93e72c6cf4c1ed19ac4339\">Update INSTALL landing redirection notice for build instructions.</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2016-10-05T23:27:23Z\" data-view-component=\"true\" class=\"no-wrap\">Oct 5, 2016</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"Makefile.am\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/Makefile.am\">Makefile.am</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Makefile.am: use APP_DIST_DIR instead of hard-coding dist\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/c090a3e9238ba2df07875b4708e908d8dca4ed9b\">Makefile.am: use APP_DIST_DIR instead of hard-coding dist</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-05-13T19:41:56Z\" data-view-component=\"true\" class=\"no-wrap\">May 13, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"README.md\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/README.md\">README.md</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"doc: Rework internal and external links\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/77772a1b809e443a6861ee49009ff8bc55cff9c3\">doc: Rework internal and external links</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-02-17T08:18:46Z\" data-view-component=\"true\" class=\"no-wrap\">Feb 17, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"REVIEWERS\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/REVIEWERS\">REVIEWERS</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Update REVIEWERS: I&#39;ve found that I keep track of PRs in need of review without the need for DrahtBot&#39;s automated notification :)\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/3636d9be8f1daf9160db84c46731b9ef8a7cbc6c\">Update REVIEWERS: I\'ve found that I keep track of PRs in need of revi…</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-10T09:00:05Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 10, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"SECURITY.md\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/SECURITY.md\">SECURITY.md</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"doc: Remove explicit mention of version from SECURITY.md\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/fa4bc4ebf91e2bf4732063f7a374a98902436a7c\">doc: Remove explicit mention of version from SECURITY.md</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2019-06-14T10:39:17Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 14, 2019</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"autogen.sh\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/autogen.sh\">autogen.sh</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"scripted-diff: Bump copyright of files changed in 2019 -BEGIN VERIFY SCRIPT- ./contrib/devtools/copyright_header.py update ./ -END VERIFY SCRIPT-\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/aaaaad6ac95b402fe18d019d67897ced6b316ee0\">scripted-diff: Bump copyright of files changed in 2019</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2019-12-29T21:42:20Z\" data-view-component=\"true\" class=\"no-wrap\">Dec 29, 2019</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"configure.ac\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/configure.ac\">configure.ac</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#22238: build: improve detection of eBPF support 8f7704d0321a71c1691837a6bd3b4e05f84d3031 build: improve detection of eBPF support (fanquake) Pull request description: Just checking for the `sys/sdt.h` header isn\'t enough, as systems like macOS have the header, but it doesn\'t actually have the `DTRACE_PROBE*` probes, which leads to [compile failures](https://github.com/bitcoin/bitcoin/pull/22006#issuecomment-859559004). The contents of `sys/sdt.h` in the macOS SDK is: ```bash #ifndef _SYS_SDT_H #define _SYS_SDT_H /* * This is a wrapper header that wraps the mach visible sdt.h header so that * the header file ends up visible where software expects it to be. We also * do the C/C++ symbol wrapping here, since Mach headers are technically C * interfaces. * * Note: The process of adding USDT probes to code is slightly different * than documented in the &quot;Solaris Dynamic Tracing Guide&quot;. * The DTRACE_PROBE*() macros are not supported on Mac OS X -- instead see * &quot;BUILDING CODE CONTAINING USDT PROBES&quot; in the dtrace(1) manpage * */ #include &lt;sys/cdefs.h&gt; __BEGIN_DECLS #include &lt;mach/sdt.h&gt; __END_DECLS #endif /* _SYS_SDT_H */ ``` The `BUILDING CODE CONTAINING USDT PROBES` section from the dtrace manpage is available [here](https://gist.github.com/fanquake/e56c9866d53b326646d04ab43a8df9e2), and outlines the more involved process of using USDT probes on macOS. ACKs for top commit: jb55: utACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031 practicalswift: cr ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031 hebasto: ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031, tested on macOS Big Sur 11.4 (20F71) and on Linux Mint 20.1 (x86_64) with depends. Tree-SHA512: 5f1351d0ac2e655fccb22a5454f415906404fdaa336fd89b54ef49ca50a442c44ab92d063cba3f161cb8ea0679c92ae3cd6cfbbcb19728cac21116247a017df5\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/ad0c8f356ee8cc4aad3ff5eef215ffe7420e0ff0\">Merge</a> <a class=\"issue-link js-issue-link\" data-error-text=\"Failed to load title\" data-id=\"920028295\" data-permission-text=\"Title is private\" data-url=\"https://github.com/bitcoin/bitcoin/issues/22238\" data-hovercard-type=\"pull_request\" data-hovercard-url=\"/bitcoin/bitcoin/pull/22238/hovercard\" href=\"https://github.com/bitcoin/bitcoin/pull/22238\">#22238</a><a data-pjax=\"true\" title=\"Merge bitcoin/bitcoin#22238: build: improve detection of eBPF support 8f7704d0321a71c1691837a6bd3b4e05f84d3031 build: improve detection of eBPF support (fanquake) Pull request description: Just checking for the `sys/sdt.h` header isn\'t enough, as systems like macOS have the header, but it doesn\'t actually have the `DTRACE_PROBE*` probes, which leads to [compile failures](https://github.com/bitcoin/bitcoin/pull/22006#issuecomment-859559004). The contents of `sys/sdt.h` in the macOS SDK is: ```bash #ifndef _SYS_SDT_H #define _SYS_SDT_H /* * This is a wrapper header that wraps the mach visible sdt.h header so that * the header file ends up visible where software expects it to be. We also * do the C/C++ symbol wrapping here, since Mach headers are technically C * interfaces. * * Note: The process of adding USDT probes to code is slightly different * than documented in the &quot;Solaris Dynamic Tracing Guide&quot;. * The DTRACE_PROBE*() macros are not supported on Mac OS X -- instead see * &quot;BUILDING CODE CONTAINING USDT PROBES&quot; in the dtrace(1) manpage * */ #include &lt;sys/cdefs.h&gt; __BEGIN_DECLS #include &lt;mach/sdt.h&gt; __END_DECLS #endif /* _SYS_SDT_H */ ``` The `BUILDING CODE CONTAINING USDT PROBES` section from the dtrace manpage is available [here](https://gist.github.com/fanquake/e56c9866d53b326646d04ab43a8df9e2), and outlines the more involved process of using USDT probes on macOS. ACKs for top commit: jb55: utACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031 practicalswift: cr ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031 hebasto: ACK 8f7704d0321a71c1691837a6bd3b4e05f84d3031, tested on macOS Big Sur 11.4 (20F71) and on Linux Mint 20.1 (x86_64) with depends. Tree-SHA512: 5f1351d0ac2e655fccb22a5454f415906404fdaa336fd89b54ef49ca50a442c44ab92d063cba3f161cb8ea0679c92ae3cd6cfbbcb19728cac21116247a017df5\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/ad0c8f356ee8cc4aad3ff5eef215ffe7420e0ff0\">: build: improve detection of eBPF support</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2021-06-18T07:16:00Z\" data-view-component=\"true\" class=\"no-wrap\">Jun 18, 2021</time-ago> </div> </div> <div role=\"row\" class=\"Box-row Box-row--focus-gray py-2 d-flex position-relative js-navigation-item \"> <div role=\"gridcell\" class=\"mr-3 flex-shrink-0\" style=\"width: 16px;\"> <svg aria-label=\"File\" aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-file color-icon-tertiary\"> <path fill-rule=\"evenodd\" d=\"M3.75 1.5a.25.25 0 00-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 00.25-.25V6H9.75A1.75 1.75 0 018 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0112.25 15h-8.5A1.75 1.75 0 012 13.25V1.75z\"></path> </svg> </div> <div role=\"rowheader\" class=\"flex-auto min-width-0 col-md-2 mr-3\"> <span class=\"css-truncate css-truncate-target d-block width-fit\"><a class=\"js-navigation-open Link--primary\" title=\"libbitcoinconsensus.pc.in\" data-pjax=\"#repo-content-pjax-container\" href=\"/bitcoin/bitcoin/blob/master/libbitcoinconsensus.pc.in\">libbitcoinconsensus.pc.in</a></span> </div> <div role=\"gridcell\" class=\"flex-auto min-width-0 d-none d-md-block col-5 mr-3\" > <span class=\"css-truncate css-truncate-target d-block width-fit markdown-title\"> <a data-pjax=\"true\" title=\"build: remove libcrypto as internal dependency in libbitcoinconsensus.pc\" class=\"Link--secondary\" href=\"/bitcoin/bitcoin/commit/2d7066527a456f8e1f4f603fe104b0bd9d864559\">build: remove libcrypto as internal dependency in libbitcoinconsensus.pc</a> </span> </div> <div role=\"gridcell\" class=\"color-text-tertiary text-right\" style=\"width:100px;\"> <time-ago datetime=\"2019-11-19T14:03:44Z\" data-view-component=\"true\" class=\"no-wrap\">Nov 19, 2019</time-ago> </div> </div> </div> <div class=\"Details-content--shown Box-footer d-md-none p-0\"> <button type=\"button\" class=\"d-block btn-link js-details-target width-full px-3 py-2\" aria-expanded=\"false\"> View code </button> </div> </div> </div> <readme-toc> <div id=\"readme\" class=\"Box md js-code-block-container Box--responsive\"> <div class=\"d-flex js-sticky js-position-sticky top-0 border-top-0 border-bottom p-2 flex-items-center flex-justify-between color-bg-primary rounded-top-2\" style=\"position: sticky; z-index: 90;\" > <div class=\"d-flex flex-items-center\"> <details data-target=\"readme-toc.trigger\" data-menu-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;trigger&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-menu-hydro-click-hmac=\"6efdafc4692df2788cc9474cb391c7072addd46122e74942558b7ab2f8d1fce1\" class=\"dropdown details-reset details-overlay\" > <summary class=\"btn btn-octicon m-0 mr-2 p-2\" aria-haspopup=\"true\" aria-label=\"Table of Contents\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-list-unordered\"> <path fill-rule=\"evenodd\" d=\"M2 4a1 1 0 100-2 1 1 0 000 2zm3.75-1.5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zm0 5a.75.75 0 000 1.5h8.5a.75.75 0 000-1.5h-8.5zM3 8a1 1 0 11-2 0 1 1 0 012 0zm-1 6a1 1 0 100-2 1 1 0 000 2z\"></path> </svg> </summary> <details-menu class=\"SelectMenu\" role=\"menu\"> <div class=\"SelectMenu-modal rounded-3 mt-1\" style=\"max-height:340px;\"> <div class=\"SelectMenu-list SelectMenu-list--borderless p-2\" style=\"overscroll-behavior: contain;\"> <a role=\"menuitem\" class=\"filter-item py-1 text-emphasized\" style=\"padding-left: 12px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#bitcoin-core-integrationstaging-tree\">Bitcoin Core integration/staging tree</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 24px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#what-is-bitcoin\">What is Bitcoin?</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 24px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#license\">License</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 24px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#development-process\">Development Process</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 24px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#testing\">Testing</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 36px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#automated-testing\">Automated Testing</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 36px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#manual-quality-assurance-qa-testing\">Manual Quality Assurance (QA) Testing</a> <a role=\"menuitem\" class=\"filter-item py-1 \" style=\"padding-left: 24px;\" data-action=\"click:readme-toc#blur\" data-targets=\"readme-toc.entries\" data-hydro-click=\"{&quot;event_type&quot;:&quot;repository_toc_menu.click&quot;,&quot;payload&quot;:{&quot;target&quot;:&quot;entry&quot;,&quot;repository_id&quot;:1181927,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"affef1e3ee81c31decbffd1b416f6282eec6c8f35f00e14562e9106038913625\" href=\"#translations\">Translations</a> </div> </div> </details-menu> </details> <h2 class=\"Box-title\"> <a href=\"#readme\" data-view-component=\"true\" class=\"Link--primary\">README.md</a> </h2> </div> </div> <div class=\"Popover anim-scale-in js-tagsearch-popover\" hidden data-tagsearch-url=\"/bitcoin/bitcoin/find-definition\" data-tagsearch-ref=\"master\" data-tagsearch-path=\"README.md\" data-tagsearch-lang=\"Markdown\" data-hydro-click=\"{&quot;event_type&quot;:&quot;code_navigation.click_on_symbol&quot;,&quot;payload&quot;:{&quot;action&quot;:&quot;click_on_symbol&quot;,&quot;repository_id&quot;:1181927,&quot;ref&quot;:&quot;master&quot;,&quot;language&quot;:&quot;Markdown&quot;,&quot;originating_url&quot;:&quot;https://github.com/bitcoin/bitcoin&quot;,&quot;user_id&quot;:null}}\" data-hydro-click-hmac=\"7cb0392b00d603ce84bc4959068c92914f7b08e0dff712cd2e70475f64a0ce8c\"> <div class=\"Popover-message Popover-message--large Popover-message--top-left TagsearchPopover mt-1 mb-4 mx-auto Box color-shadow-large\"> <div class=\"TagsearchPopover-content js-tagsearch-popover-content overflow-auto\" style=\"will-change:transform;\"> </div> </div> </div> <div data-target=\"readme-toc.content\" class=\"Box-body px-5 pb-5\"> <article class=\"markdown-body entry-content container-lg\" itemprop=\"text\"><h1><a id=\"user-content-bitcoin-core-integrationstaging-tree\" class=\"anchor\" aria-hidden=\"true\" href=\"#bitcoin-core-integrationstaging-tree\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>Bitcoin Core integration/staging tree</h1> <p><a href=\"https://bitcoincore.org\" rel=\"nofollow\">https://bitcoincore.org</a></p> <p>For an immediately usable, binary version of the Bitcoin Core software, see <a href=\"https://bitcoincore.org/en/download/\" rel=\"nofollow\">https://bitcoincore.org/en/download/</a>.</p> <p>Further information about Bitcoin Core is available in the <a href=\"/bitcoin/bitcoin/blob/master/doc\">doc folder</a>.</p> <h2><a id=\"user-content-what-is-bitcoin\" class=\"anchor\" aria-hidden=\"true\" href=\"#what-is-bitcoin\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>What is Bitcoin?</h2> <p>Bitcoin is an experimental digital currency that enables instant payments to anyone, anywhere in the world. Bitcoin uses peer-to-peer technology to operate with no central authority: managing transactions and issuing money are carried out collectively by the network. Bitcoin Core is the name of open source software which enables the use of this currency.</p> <p>For more information read the original Bitcoin whitepaper.</p> <h2><a id=\"user-content-license\" class=\"anchor\" aria-hidden=\"true\" href=\"#license\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>License</h2> <p>Bitcoin Core is released under the terms of the MIT license. See <a href=\"/bitcoin/bitcoin/blob/master/COPYING\">COPYING</a> for more information or see <a href=\"https://opensource.org/licenses/MIT\" rel=\"nofollow\">https://opensource.org/licenses/MIT</a>.</p> <h2><a id=\"user-content-development-process\" class=\"anchor\" aria-hidden=\"true\" href=\"#development-process\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>Development Process</h2> <p>The <code>master</code> branch is regularly built (see <code>doc/build-*.md</code> for instructions) and tested, but it is not guaranteed to be completely stable. <a href=\"https://github.com/bitcoin/bitcoin/tags\">Tags</a> are created regularly from release branches to indicate new official, stable release versions of Bitcoin Core.</p> <p>The <a href=\"https://github.com/bitcoin-core/gui\">https://github.com/bitcoin-core/gui</a> repository is used exclusively for the development of the GUI. Its master branch is identical in all monotree repositories. Release branches and tags do not exist, so please do not fork that repository unless it is for development reasons.</p> <p>The contribution workflow is described in <a href=\"/bitcoin/bitcoin/blob/master/CONTRIBUTING.md\">CONTRIBUTING.md</a> and useful hints for developers can be found in <a href=\"/bitcoin/bitcoin/blob/master/doc/developer-notes.md\">doc/developer-notes.md</a>.</p> <h2><a id=\"user-content-testing\" class=\"anchor\" aria-hidden=\"true\" href=\"#testing\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>Testing</h2> <p>Testing and code review is the bottleneck for development; we get more pull requests than we can review and test on short notice. Please be patient and help out by testing other people\'s pull requests, and remember this is a security-critical project where any mistake might cost people lots of money.</p> <h3><a id=\"user-content-automated-testing\" class=\"anchor\" aria-hidden=\"true\" href=\"#automated-testing\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>Automated Testing</h3> <p>Developers are strongly encouraged to write <a href=\"/bitcoin/bitcoin/blob/master/src/test/README.md\">unit tests</a> for new code, and to submit new unit tests for old code. Unit tests can be compiled and run (assuming they weren\'t disabled in configure) with: <code>make check</code>. Further details on running and extending unit tests can be found in <a href=\"/bitcoin/bitcoin/blob/master/src/test/README.md\">/src/test/README.md</a>.</p> <p>There are also <a href=\"/bitcoin/bitcoin/blob/master/test\">regression and integration tests</a>, written in Python. These tests can be run (if the <a href=\"/bitcoin/bitcoin/blob/master/test\">test dependencies</a> are installed) with: <code>test/functional/test_runner.py</code></p> <p>The CI (Continuous Integration) systems make sure that every pull request is built for Windows, Linux, and macOS, and that unit/sanity tests are run automatically.</p> <h3><a id=\"user-content-manual-quality-assurance-qa-testing\" class=\"anchor\" aria-hidden=\"true\" href=\"#manual-quality-assurance-qa-testing\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>Manual Quality Assurance (QA) Testing</h3> <p>Changes should be tested by somebody other than the developer who wrote the code. This is especially important for large or high-risk changes. It is useful to add a test plan to the pull request description if testing the changes is not straightforward.</p> <h2><a id=\"user-content-translations\" class=\"anchor\" aria-hidden=\"true\" href=\"#translations\"><svg class=\"octicon octicon-link\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path></svg></a>Translations</h2> <p>Changes to translations as well as new translations can be submitted to <a href=\"https://www.transifex.com/bitcoin/bitcoin/\" rel=\"nofollow\">Bitcoin Core\'s Transifex page</a>.</p> <p>Translations are periodically pulled from Transifex and merged into the git repository. See the <a href=\"/bitcoin/bitcoin/blob/master/doc/translation_process.md\">translation process</a> for details on how this works.</p> <p><strong>Important</strong>: We do not accept translation changes as GitHub pull requests because the next pull from Transifex would automatically overwrite them again.</p> </article> </div> </div> </readme-toc> </div> <div data-view-component=\"true\" class=\"flex-shrink-0 col-12 col-md-3\"> <div class=\"BorderGrid BorderGrid--spacious\" data-pjax> <div class=\"BorderGrid-row hide-sm hide-md\"> <div class=\"BorderGrid-cell\"> <h2 class=\"mb-3 h4\">About</h2> <p class=\"f4 mt-3\"> Bitcoin Core integration/staging tree </p> <div class=\"mt-3 d-flex flex-items-center\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-link flex-shrink-0 mr-2\"> <path fill-rule=\"evenodd\" d=\"M7.775 3.275a.75.75 0 001.06 1.06l1.25-1.25a2 2 0 112.83 2.83l-2.5 2.5a2 2 0 01-2.83 0 .75.75 0 00-1.06 1.06 3.5 3.5 0 004.95 0l2.5-2.5a3.5 3.5 0 00-4.95-4.95l-1.25 1.25zm-4.69 9.64a2 2 0 010-2.83l2.5-2.5a2 2 0 012.83 0 .75.75 0 001.06-1.06 3.5 3.5 0 00-4.95 0l-2.5 2.5a3.5 3.5 0 004.95 4.95l1.25-1.25a.75.75 0 00-1.06-1.06l-1.25 1.25a2 2 0 01-2.83 0z\"></path> </svg> <span class=\"flex-auto min-width-0 css-truncate css-truncate-target width-fit\"> <a title=\"https://bitcoincore.org/en/download\" role=\"link\" target=\"_blank\" class=\"text-bold\" rel=\"noopener noreferrer\" href=\"https://bitcoincore.org/en/download\">bitcoincore.org/en/download</a> </span> </div> <h3 class=\"sr-only\">Topics</h3> <div class=\"mt-3\"> <div class=\"f6\"> <a data-ga-click=\"Topic, repository page\" data-octo-click=\"topic_click\" data-octo-dimensions=\"topic:c-plus-plus\" href=\"/topics/c-plus-plus\" title=\"Topic: c-plus-plus\" data-view-component=\"true\" class=\"topic-tag topic-tag-link\"> c-plus-plus </a> <a data-ga-click=\"Topic, repository page\" data-octo-click=\"topic_click\" data-octo-dimensions=\"topic:cryptography\" href=\"/topics/cryptography\" title=\"Topic: cryptography\" data-view-component=\"true\" class=\"topic-tag topic-tag-link\"> cryptography </a> <a data-ga-click=\"Topic, repository page\" data-octo-click=\"topic_click\" data-octo-dimensions=\"topic:bitcoin\" href=\"/topics/bitcoin\" title=\"Topic: bitcoin\" data-view-component=\"true\" class=\"topic-tag topic-tag-link\"> bitcoin </a> <a data-ga-click=\"Topic, repository page\" data-octo-click=\"topic_click\" data-octo-dimensions=\"topic:p2p\" href=\"/topics/p2p\" title=\"Topic: p2p\" data-view-component=\"true\" class=\"topic-tag topic-tag-link\"> p2p </a> <a data-ga-click=\"Topic, repository page\" data-octo-click=\"topic_click\" data-octo-dimensions=\"topic:cryptocurrency\" href=\"/topics/cryptocurrency\" title=\"Topic: cryptocurrency\" data-view-component=\"true\" class=\"topic-tag topic-tag-link\"> cryptocurrency </a> </div> </div> <h3 class=\"sr-only\">Resources</h3> <div class=\"mt-3\"> <a class=\"Link--muted\" href=\"#readme\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-book mr-2\"> <path fill-rule=\"evenodd\" d=\"M0 1.75A.75.75 0 01.75 1h4.253c1.227 0 2.317.59 3 1.501A3.744 3.744 0 0111.006 1h4.245a.75.75 0 01.75.75v10.5a.75.75 0 01-.75.75h-4.507a2.25 2.25 0 00-1.591.659l-.622.621a.75.75 0 01-1.06 0l-.622-.621A2.25 2.25 0 005.258 13H.75a.75.75 0 01-.75-.75V1.75zm8.755 3a2.25 2.25 0 012.25-2.25H14.5v9h-3.757c-.71 0-1.4.201-1.992.572l.004-7.322zm-1.504 7.324l.004-5.073-.002-2.253A2.25 2.25 0 005.003 2.5H1.5v9h3.757a3.75 3.75 0 011.994.574z\"></path> </svg> Readme </a> </div> <h3 class=\"sr-only\">License</h3> <div class=\"mt-3\"> <a href=\"/bitcoin/bitcoin/blob/master/COPYING\" class=\"Link--muted\" > <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-law mr-2\"> <path fill-rule=\"evenodd\" d=\"M8.75.75a.75.75 0 00-1.5 0V2h-.984c-.305 0-.604.08-.869.23l-1.288.737A.25.25 0 013.984 3H1.75a.75.75 0 000 1.5h.428L.066 9.192a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.514 3.514 0 00.686.45A4.492 4.492 0 003 11c.88 0 1.556-.22 2.023-.454a3.515 3.515 0 00.686-.45l.045-.04.016-.015.006-.006.002-.002.001-.002L5.25 9.5l.53.53a.75.75 0 00.154-.838L3.822 4.5h.162c.305 0 .604-.08.869-.23l1.289-.737a.25.25 0 01.124-.033h.984V13h-2.5a.75.75 0 000 1.5h6.5a.75.75 0 000-1.5h-2.5V3.5h.984a.25.25 0 01.124.033l1.29.736c.264.152.563.231.868.231h.162l-2.112 4.692a.75.75 0 00.154.838l.53-.53-.53.53v.001l.002.002.002.002.006.006.016.015.045.04a3.517 3.517 0 00.686.45A4.492 4.492 0 0013 11c.88 0 1.556-.22 2.023-.454a3.512 3.512 0 00.686-.45l.045-.04.01-.01.006-.005.006-.006.002-.002.001-.002-.529-.531.53.53a.75.75 0 00.154-.838L13.823 4.5h.427a.75.75 0 000-1.5h-2.234a.25.25 0 01-.124-.033l-1.29-.736A1.75 1.75 0 009.735 2H8.75V.75zM1.695 9.227c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L3 6.327l-1.305 2.9zm10 0c.285.135.718.273 1.305.273s1.02-.138 1.305-.273L13 6.327l-1.305 2.9z\"></path> </svg> MIT License </a> </div> </div> </div> <div class=\"BorderGrid-row\"> <div class=\"BorderGrid-cell\"> <h2 class=\"h4 mb-3\"> <a href=\"/bitcoin/bitcoin/releases\" data-view-component=\"true\" class=\"Link--primary no-underline\"> Releases <span title=\"249\" data-view-component=\"true\" class=\"Counter\">249</span> </a></h2> <a class=\"Link--primary d-flex no-underline\" href=\"/bitcoin/bitcoin/releases/tag/v0.21.1\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-tag flex-shrink-0 mt-1 color-text-success\"> <path fill-rule=\"evenodd\" d=\"M2.5 7.775V2.75a.25.25 0 01.25-.25h5.025a.25.25 0 01.177.073l6.25 6.25a.25.25 0 010 .354l-5.025 5.025a.25.25 0 01-.354 0l-6.25-6.25a.25.25 0 01-.073-.177zm-1.5 0V2.75C1 1.784 1.784 1 2.75 1h5.025c.464 0 .91.184 1.238.513l6.25 6.25a1.75 1.75 0 010 2.474l-5.026 5.026a1.75 1.75 0 01-2.474 0l-6.25-6.25A1.75 1.75 0 011 7.775zM6 5a1 1 0 100 2 1 1 0 000-2z\"></path> </svg> <div class=\"ml-2 min-width-0\"> <div class=\"d-flex\"> <span class=\"css-truncate css-truncate-target text-bold mr-2\" style=\"max-width: none;\">Bitcoin Core 0.21.1</span> <span title=\"Label: Latest\" data-view-component=\"true\" class=\"Label Label--success flex-shrink-0\"> Latest </span> </div> <div class=\"text-small color-text-secondary\"><relative-time datetime=\"2021-05-03T01:33:04Z\" class=\"no-wrap\">May 3, 2021</relative-time></div> </div> </a> <div data-view-component=\"true\" class=\"mt-3\"> <a href=\"/bitcoin/bitcoin/releases\" data-view-component=\"true\" class=\"text-small\"> + 248 releases </a></div> </div> </div> <div class=\"BorderGrid-row\"> <div class=\"BorderGrid-cell\"> <h2 class=\"h4 mb-3\"> <a href=\"/orgs/bitcoin/packages?repo_name=bitcoin\" data-view-component=\"true\" class=\"Link--primary no-underline\"> Packages <span title=\"0\" hidden=\"hidden\" data-view-component=\"true\" class=\"Counter\">0</span> </a></h2> <div class=\"text-small color-text-secondary\"> No packages published <br> </div> </div> </div> <div class=\"BorderGrid-row\"> <div class=\"BorderGrid-cell\"> <h2 class=\"h4 mb-3\"> <a href=\"/bitcoin/bitcoin/graphs/contributors\" data-view-component=\"true\" class=\"Link--primary no-underline\"> Contributors <span title=\"802\" data-view-component=\"true\" class=\"Counter\">802</span> </a></h2> <ul class=\"list-style-none d-flex flex-wrap mb-n2\"> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/laanwj/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/laanwj\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/126646?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@laanwj\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/MarcoFalke/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/MarcoFalke\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/6399679?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@MarcoFalke\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/sipa/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/sipa\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/548488?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@sipa\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/fanquake/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/fanquake\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/863730?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@fanquake\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/gavinandresen/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/gavinandresen\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/331997?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@gavinandresen\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/jonasschnelli/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/jonasschnelli\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/178464?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@jonasschnelli\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/hebasto/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/hebasto\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/32963518?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@hebasto\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/practicalswift/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/practicalswift\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/7826565?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@practicalswift\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/jnewbery/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/jnewbery\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/1063656?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@jnewbery\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/TheBlueMatt/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/TheBlueMatt\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/649246?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@TheBlueMatt\" /> </a> </li> <li class=\"mb-2 mr-2\"> <a class=\"\" data-hovercard-type=\"user\" data-hovercard-url=\"/users/theuni/hovercard\" data-octo-click=\"hovercard-link-click\" data-octo-dimensions=\"link_type:self\" href=\"/theuni\"> <img class=\"d-block avatar-user\" src=\"https://avatars.githubusercontent.com/u/417043?s=64&amp;v=4\" width=\"32\" height=\"32\" alt=\"@theuni\" /> </a> </li> </ul> <div data-view-component=\"true\" class=\"mt-3\"> <a href=\"/bitcoin/bitcoin/graphs/contributors\" data-view-component=\"true\" class=\"text-small\"> + 791 contributors </a></div> </div> </div> <div class=\"BorderGrid-row\"> <div class=\"BorderGrid-cell\"> <h2 class=\"h4 mb-3\">Languages</h2> <div class=\"mb-2\"> <span data-view-component=\"true\" class=\"Progress\"> <span itemprop=\"keywords\" aria-label=\"C++ 66.6\" style=\"background-color: #f34b7d;width: 66.6%;\" data-view-component=\"true\" class=\"Progress-item\"></span> <span itemprop=\"keywords\" aria-label=\"Python 18.9\" style=\"background-color: #3572A5;width: 18.9%;\" data-view-component=\"true\" class=\"Progress-item\"></span> <span itemprop=\"keywords\" aria-label=\"C 9.2\" style=\"background-color: #555555;width: 9.2%;\" data-view-component=\"true\" class=\"Progress-item\"></span> <span itemprop=\"keywords\" aria-label=\"M4 1.6\" style=\"background-color: #ccc;width: 1.6%;\" data-view-component=\"true\" class=\"Progress-item\"></span> <span itemprop=\"keywords\" aria-label=\"Shell 1.6\" style=\"background-color: #89e051;width: 1.6%;\" data-view-component=\"true\" class=\"Progress-item\"></span> <span itemprop=\"keywords\" aria-label=\"Makefile 1.0\" style=\"background-color: #427819;width: 1.0%;\" data-view-component=\"true\" class=\"Progress-item\"></span> <span itemprop=\"keywords\" aria-label=\"Other 1.1\" style=\"background-color: #ededed;width: 1.1%;\" data-view-component=\"true\" class=\"Progress-item\"></span> </span></div> <ul class=\"list-style-none\"> <li class=\"d-inline\"> <a class=\"d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3\" href=\"/bitcoin/bitcoin/search?l=c%2B%2B\" data-ga-click=\"Repository, language stats search click, location:repo overview\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#f34b7d;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">C++</span> <span>66.6%</span> </a> </li> <li class=\"d-inline\"> <a class=\"d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3\" href=\"/bitcoin/bitcoin/search?l=python\" data-ga-click=\"Repository, language stats search click, location:repo overview\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#3572A5;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">Python</span> <span>18.9%</span> </a> </li> <li class=\"d-inline\"> <a class=\"d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3\" href=\"/bitcoin/bitcoin/search?l=c\" data-ga-click=\"Repository, language stats search click, location:repo overview\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#555555;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">C</span> <span>9.2%</span> </a> </li> <li class=\"d-inline\"> <a class=\"d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3\" href=\"/bitcoin/bitcoin/search?l=m4\" data-ga-click=\"Repository, language stats search click, location:repo overview\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#ccc;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">M4</span> <span>1.6%</span> </a> </li> <li class=\"d-inline\"> <a class=\"d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3\" href=\"/bitcoin/bitcoin/search?l=shell\" data-ga-click=\"Repository, language stats search click, location:repo overview\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#89e051;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">Shell</span> <span>1.6%</span> </a> </li> <li class=\"d-inline\"> <a class=\"d-inline-flex flex-items-center flex-nowrap Link--secondary no-underline text-small mr-3\" href=\"/bitcoin/bitcoin/search?l=makefile\" data-ga-click=\"Repository, language stats search click, location:repo overview\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#427819;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">Makefile</span> <span>1.0%</span> </a> </li> <li class=\"d-inline\"> <span class=\"d-inline-flex flex-items-center flex-nowrap text-small mr-3\"> <svg class=\"octicon octicon-dot-fill mr-2\" style=\"color:#ededed;\" viewBox=\"0 0 16 16\" version=\"1.1\" width=\"16\" height=\"16\" aria-hidden=\"true\"><path fill-rule=\"evenodd\" d=\"M8 4a4 4 0 100 8 4 4 0 000-8z\"></path></svg> <span class=\"color-text-primary text-bold mr-1\">Other</span> <span>1.1%</span> </span> </li> </ul> </div> </div> </div> </div> </div></div> </div> </div> </main> </div> </div> <div class=\"footer container-xl width-full p-responsive\" role=\"contentinfo\"> <div class=\"position-relative d-flex flex-row-reverse flex-lg-row flex-wrap flex-lg-nowrap flex-justify-center flex-lg-justify-between pt-6 pb-2 mt-6 f6 color-text-secondary border-top color-border-secondary \"> <ul class=\"list-style-none d-flex flex-wrap col-12 col-lg-5 flex-justify-center flex-lg-justify-between mb-2 mb-lg-0\"> <li class=\"mr-3 mr-lg-0\">&copy; 2021 GitHub, Inc.</li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://docs.github.com/en/github/site-policy/github-terms-of-service\" data-ga-click=\"Footer, go to terms, text:terms\">Terms</a></li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://docs.github.com/en/github/site-policy/github-privacy-statement\" data-ga-click=\"Footer, go to privacy, text:privacy\">Privacy</a></li> <li class=\"mr-3 mr-lg-0\"><a data-ga-click=\"Footer, go to security, text:security\" href=\"https://github.com/security\">Security</a></li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://www.githubstatus.com/\" data-ga-click=\"Footer, go to status, text:status\">Status</a></li> <li><a data-ga-click=\"Footer, go to help, text:Docs\" href=\"https://docs.github.com\">Docs</a></li> </ul> <a aria-label=\"Homepage\" title=\"GitHub\" class=\"footer-octicon d-none d-lg-block mx-lg-4\" href=\"https://github.com\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"24\" width=\"24\" class=\"octicon octicon-mark-github\"> <path fill-rule=\"evenodd\" d=\"M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z\"></path> </svg> </a> <ul class=\"list-style-none d-flex flex-wrap col-12 col-lg-5 flex-justify-center flex-lg-justify-between mb-2 mb-lg-0\"> <li class=\"mr-3 mr-lg-0\"><a href=\"https://support.github.com\" data-ga-click=\"Footer, go to contact, text:contact\">Contact GitHub</a></li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://github.com/pricing\" data-ga-click=\"Footer, go to Pricing, text:Pricing\">Pricing</a></li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://docs.github.com\" data-ga-click=\"Footer, go to api, text:api\">API</a></li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://services.github.com\" data-ga-click=\"Footer, go to training, text:training\">Training</a></li> <li class=\"mr-3 mr-lg-0\"><a href=\"https://github.blog\" data-ga-click=\"Footer, go to blog, text:blog\">Blog</a></li> <li><a data-ga-click=\"Footer, go to about, text:about\" href=\"https://github.com/about\">About</a></li> </ul> </div> <div class=\"d-flex flex-justify-center pb-6\"> <span class=\"f6 color-text-tertiary\"></span> </div> </div> <div id=\"ajax-error-message\" class=\"ajax-error-message flash flash-error\" hidden> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-alert\"> <path fill-rule=\"evenodd\" d=\"M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z\"></path> </svg> <button type=\"button\" class=\"flash-close js-ajax-error-dismiss\" aria-label=\"Dismiss error\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-x\"> <path fill-rule=\"evenodd\" d=\"M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z\"></path> </svg> </button> You can’t perform that action at this time. </div> <div class=\"js-stale-session-flash flash flash-warn flash-banner\" hidden > <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-alert\"> <path fill-rule=\"evenodd\" d=\"M8.22 1.754a.25.25 0 00-.44 0L1.698 13.132a.25.25 0 00.22.368h12.164a.25.25 0 00.22-.368L8.22 1.754zm-1.763-.707c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 00-1.5 0v2.5a.75.75 0 001.5 0v-2.5z\"></path> </svg> <span class=\"js-stale-session-flash-signed-in\" hidden>You signed in with another tab or window. <a href=\"\">Reload</a> to refresh your session.</span> <span class=\"js-stale-session-flash-signed-out\" hidden>You signed out in another tab or window. <a href=\"\">Reload</a> to refresh your session.</span> </div> <template id=\"site-details-dialog\"> <details class=\"details-reset details-overlay details-overlay-dark lh-default color-text-primary hx_rsm\" open> <summary role=\"button\" aria-label=\"Close dialog\"></summary> <details-dialog class=\"Box Box--overlay d-flex flex-column anim-fade-in fast hx_rsm-dialog hx_rsm-modal\"> <button class=\"Box-btn-octicon m-0 btn-octicon position-absolute right-0 top-0\" type=\"button\" aria-label=\"Close dialog\" data-close-dialog> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-x\"> <path fill-rule=\"evenodd\" d=\"M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z\"></path> </svg> </button> <div class=\"octocat-spinner my-6 js-details-dialog-spinner\"></div> </details-dialog> </details> </template> <div class=\"Popover js-hovercard-content position-absolute\" style=\"display: none; outline: none;\" tabindex=\"0\"> <div class=\"Popover-message Popover-message--bottom-left Popover-message--large Box color-shadow-large\" style=\"width:360px;\"> </div> </div> <template id=\"snippet-clipboard-copy-button\"> <div class=\"zeroclipboard-container position-absolute right-0 top-0\"> <clipboard-copy aria-label=\"Copy\" class=\"ClipboardButton btn js-clipboard-copy m-2 p-0 tooltipped-no-delay\" data-copy-feedback=\"Copied!\" data-tooltip-direction=\"w\"> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-clippy js-clipboard-clippy-icon m-2\"> <path fill-rule=\"evenodd\" d=\"M5.75 1a.75.75 0 00-.75.75v3c0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75v-3a.75.75 0 00-.75-.75h-4.5zm.75 3V2.5h3V4h-3zm-2.874-.467a.75.75 0 00-.752-1.298A1.75 1.75 0 002 3.75v9.5c0 .966.784 1.75 1.75 1.75h8.5A1.75 1.75 0 0014 13.25v-9.5a1.75 1.75 0 00-.874-1.515.75.75 0 10-.752 1.298.25.25 0 01.126.217v9.5a.25.25 0 01-.25.25h-8.5a.25.25 0 01-.25-.25v-9.5a.25.25 0 01.126-.217z\"></path> </svg> <svg aria-hidden=\"true\" viewBox=\"0 0 16 16\" version=\"1.1\" data-view-component=\"true\" height=\"16\" width=\"16\" class=\"octicon octicon-check js-clipboard-check-icon color-text-success d-none m-2\"> <path fill-rule=\"evenodd\" d=\"M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z\"></path> </svg> </clipboard-copy> </div> </template> </body> </html>"