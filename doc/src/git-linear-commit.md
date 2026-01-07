# git

## why using linear commit ?

- At frame 9, we want to commit the devs made in develop branch, to the main branch. The problem is that changes were made to the main branch. One solution is to merge d3 and m5. This solution is a problem :
- can you really trust merge without going through a whole test cycle ?
- appart from cicd, m6 was never really tested.
- the fully tested version d3 is lost when develop branch is deleted.


## solution : rebase

- if you activate linear commit in github, the previous scenario will not be possible : you need to rebase before pushing a MR
- if you rebase, there is no need for a merge, because the new head of main will be exactly the head of develop
- d3 == m6 : the new head of main will be fully tested
- this version will always be available in the main branch history


```mermaid
---

config:
  flowchart:
    defaultRenderer: elk

title: Example Git diagram

animate-yml-file: git-animate.yml

---

flowchart LR

subgraph main branch
    m1 
    m2 
    m3 
    m4 
    m5 
    m6
    mr6
end

subgraph develop branch
    d1 
    d2 
    d3
    dr1
    dr2
    dr3
end

d3 ed3dr1@-->dr1



m1 em1m2@-->m2
m2 em2m3@-->m3
m3 em3m4@-->m4
m4 em4m5@-->m5
m5 em5m6@-->m6

d1 ed1d2@-->d2
d2 ed2d3@-->d3
d3 ed3m6@-->m6
dr1 edr1dr2@-->dr2
dr2 edr2dr3@-->dr3
dr3 edr3mr6@-->mr6

m3 em3d1@-->d1
m5 em5dr1@-->dr1
m5 em5mr6@-->mr6

m1((m1)) ;
m2((m2)) ;
m3((m3)) ;
m4((m4)) ;
m5((m5)) ;
m6((m6)) ;
mr6((m6)) ;

d1((d1)) ;
d2((d2)) ;
d3((d3)) ;
dr1((d1)) ;
dr2((d2)) ;
dr3((d3)) ;

classDef class_active_node      stroke-width:1px,color:black,stroke:black ;
classDef class_from_node    stroke:green,stroke-width:5px,color:black ;
classDef class_to_node      stroke:red,stroke-width:5px,color:black ;
classDef class_hidden_node     stroke-width:1px,color:white,stroke:white ;
classDef class_active_edge    stroke-width:1px,color:black,stroke:black;
classDef class_hidden_edge stroke-width:1px,stroke:white ;
classDef class_animate_edge stroke-dasharray: 9,5,stroke-dashoffset: 900,animation: dash 25s linear infinite,color black;

classDef class_old     stroke:#ccc,fill:#eee,color:#ccc;
classDef class_old_edge stroke-width:1px,stroke:#ccc,troke-dasharray: 9,5,stroke-dashoffset: 900 ;
class ed3d1r class_no_edge; 

class em5mr6 class_no_edge;

%% mermaid-animate-tag m1
%% mermaid-animate-tag m2
%% mermaid-animate-tag m3
%% mermaid-animate-tag m4
%% mermaid-animate-tag m5
%% mermaid-animate-tag m6
%% mermaid-animate-tag mr6

%% mermaid-animate-tag d1
%% mermaid-animate-tag d2
%% mermaid-animate-tag d3
%% mermaid-animate-tag dr1 
%% mermaid-animate-tag dr2 
%% mermaid-animate-tag dr3  


%% mermaid-animate-tag em1m2
%% mermaid-animate-tag em2m3
%% mermaid-animate-tag em3m4
%% mermaid-animate-tag em4m5
%% mermaid-animate-tag em5m6
%% mermaid-animate-tag em3d1
%% mermaid-animate-tag ed1d2
%% mermaid-animate-tag ed2d3
%% mermaid-animate-tag ed3dr1
%% mermaid-animate-tag ed3dr1
%% mermaid-animate-tag edr1dr2
%% mermaid-animate-tag edr2dr3
%% mermaid-animate-tag edr3m6
%% mermaid-animate-tag em5dr1
%% mermaid-animate-tag edr3mr6
%% mermaid-animate-tag em5mr6

%% mermaid-animate-tag ed3m6
%% mermaid-animate-tag em5d1

```




