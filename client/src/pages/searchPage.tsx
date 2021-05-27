import {makeStyles} from '@material-ui/core/styles';
import * as React from 'react';
import {useState} from 'react';
import SearchBar from '../components/searchBar';
import ShowCard, {ShowFormat, ShowInfo} from '../components/showCard';
import {getPodcasts} from '../api';

const useStyles = makeStyles({
  root: {
    margin: '10px',
    textAlign: 'center'
  },
  nested: {
    maxWidth: 800,
    margin: 'auto',
    textAlign: 'initial'
  },
  showCardWrapper: {
    padding: '10px 0 0 0'
  }
});

const SearchPage = () => {
  const classes = useStyles();

  const [isSearching, setIsSearching] = useState(false);
  const [podcasts, setPodcasts] = useState([] as ShowInfo[]);

  return (
    <div className={classes.root}>
      <div className={classes.nested}>
        <SearchBar onSearch={async (query) => {
          if (!isSearching) {
            setIsSearching(true);
            setPodcasts(await getPodcasts(query, 50, 0));
            setIsSearching(false);
          }
        }}/>
      </div>
      <div className={classes.nested}>
        {podcasts.map((show) => <div className={classes.showCardWrapper}><ShowCard show={show}/></div>)}
      </div>
    </div>
  );
};

export default SearchPage;