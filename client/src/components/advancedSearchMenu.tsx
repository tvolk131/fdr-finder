import {CircularProgress, Chip, Divider, TextField} from '@material-ui/core';
import {createStyles, makeStyles, Theme} from '@material-ui/core/styles';
import * as React from 'react';
import {useState, useEffect} from 'react';
import {getFilteredTagsWithCounts} from '../api';
import {getTagDisplayText} from '../helper/tagFormatting';

const useStyles = makeStyles((theme: Theme) => (
  createStyles({
    root: {
      padding: '15px 5px 5px 15px'
    },
    divider: {
      margin: '5px 0'
    },
    tagChip: {
      margin: theme.spacing(0.5)
    },
    tagSearchFieldWrapper: {
      display: 'block',
      textAlign: 'center',
      paddingBottom: theme.spacing(0.5)
    }
  })
));

const maxVisibleTags = 50;

interface AdvancedSearchMenuProps {
  searchTags: string[]
  setSearchTags: (tags: string[]) => void
}

const AdvancedSearchMenu = ({searchTags, setSearchTags}: AdvancedSearchMenuProps) => {
  const [tagFilter, setTagFilter] = useState('');
  const [tagsWithCounts, setTagsWithCounts] = useState<{tag: string, count: number}[]>([]);
  const [isLoadingTags, setIsLoadingTags] = useState(true);

  const classes = useStyles();

  useEffect(() => {
    setSearchTags(searchTags);
    setIsLoadingTags(true);
    getFilteredTagsWithCounts(searchTags).then((tagsWithCounts) => {
      tagsWithCounts.sort((a, b) => {
        if (a.count < b.count) {
          return 1;
        } else if (a.count > b.count) {
          return -1;
        } else if (a.tag > b.tag) {
          return 1;
        } else if (a.tag < b.tag) {
          return -1;
        } else {
          return 0;
        }
      });
      setTagsWithCounts(tagsWithCounts);
      setIsLoadingTags(false);
    });
  }, [searchTags]);

  const getSelectableTagChips = () => {
    const filteredTags = tagsWithCounts.filter(({tag}) => getTagDisplayText(tag).toLowerCase().includes(tagFilter.toLowerCase()))
    
    const tagChips = filteredTags.slice(0, maxVisibleTags).map(({tag, count}) => (
      <Chip
        onClick={() => setSearchTags([...searchTags, tag])}
        className={classes.tagChip}
        label={`${getTagDisplayText(tag)} (${count})`}
      />
    ));

    const nonVisibleTagCount = filteredTags.length - maxVisibleTags;

    if (nonVisibleTagCount > 0) {
      tagChips.push(<Chip label={`... +${nonVisibleTagCount}`}/>);
    }

    return tagChips;
  };

  return (
    <div>
      {!!searchTags.length && searchTags.map((tag) => (
        <Chip
          onDelete={() => setSearchTags(searchTags.filter((iterTag) => tag !== iterTag))}
          className={classes.tagChip}
          label={getTagDisplayText(tag)}
        />
      ))}
      {!!searchTags.length && <Divider className={classes.divider}/>}
      <div className={classes.tagSearchFieldWrapper}>
        <TextField value={tagFilter} onChange={(e) => setTagFilter(e.target.value)} label={'Tag Filter'}/>
      </div>
      {isLoadingTags ? <CircularProgress/> : getSelectableTagChips()}
    </div>
  );
};

export default AdvancedSearchMenu;